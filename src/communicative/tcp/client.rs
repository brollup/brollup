use crate::list::ListCodec;
use crate::{
    noist_vse,
    tcp::{self, TCPError},
};
use async_trait::async_trait;
use chrono::Utc;
use colored::Colorize;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::Mutex;

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;
type NostrClient = Arc<Mutex<nostr_sdk::Client>>;

#[derive(Copy, Clone, PartialEq)]
pub enum PeerKind {
    Node,
    Operator,
    Coordinator,
}

impl PeerKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            PeerKind::Node => "Node",
            PeerKind::Operator => "Operator",
            PeerKind::Coordinator => "Coordinator",
        }
    }
}

#[derive(Clone)]
pub struct Peer {
    kind: PeerKind,
    nns_key: [u8; 32],
    nostr_client: NostrClient,
    connection: Option<(TCPSocket, SocketAddr)>,
}

impl Peer {
    pub async fn connect(
        kind: PeerKind,
        nns_key: [u8; 32],
        nostr_client: &NostrClient,
    ) -> Result<Arc<Mutex<Self>>, TCPError> {
        let (socket_, addr) = {
            match tcp::connect_nns(nns_key, &nostr_client).await {
                Ok(socket) => {
                    let addr = match socket.peer_addr() {
                        Ok(addr) => addr,
                        Err(_) => return Err(TCPError::ConnErr),
                    };

                    (socket, addr)
                }
                Err(_) => return Err(TCPError::ConnErr),
            }
        };

        let socket: TCPSocket = Arc::new(Mutex::new(socket_));

        let connection = Some((socket, addr));

        let peer_ = Peer {
            kind,
            nns_key,
            connection,
            nostr_client: Arc::clone(nostr_client),
        };

        let peer = Arc::new(Mutex::new(peer_));

        peer.set_uptimer().await;

        Ok(peer)
    }

    pub fn kind(&self) -> PeerKind {
        self.kind
    }

    pub fn nns_key(&self) -> [u8; 32] {
        self.nns_key
    }

    pub fn nostr_client(&self) -> NostrClient {
        Arc::clone(&self.nostr_client)
    }

    pub fn connection(&self) -> Option<(TCPSocket, SocketAddr)> {
        self.connection.clone()
    }

    pub fn socket(&self) -> Option<TCPSocket> {
        let socket = Arc::clone(&self.connection()?.0);
        Some(socket)
    }

    pub fn set_connection(&mut self, connection: Option<(TCPSocket, SocketAddr)>) {
        self.connection = connection;
    }

    pub fn addr(&self) -> String {
        match self.connection() {
            Some(connection) => {
                return format!("{}:{}", connection.1.ip(), connection.1.port());
            }
            None => {
                return "Dead.".to_string();
            }
        };
    }
}

#[async_trait]
pub trait Connection {
    async fn disconnection(&self);
    async fn reconnect(&self);
    async fn set_uptimer(&self);
}

#[async_trait]
impl Connection for Arc<Mutex<Peer>> {
    async fn disconnection(&self) {
        loop {
            loop {
                match self.ping().await {
                    Ok(_) => break,
                    Err(_) => {
                        let mut failure_iter: u8 = 0;
                        loop {
                            if failure_iter < 3 {
                                failure_iter += 1;
                                tokio::time::sleep(Duration::from_secs(3)).await;
                                continue;
                            } else {
                                let mut _peer = self.lock().await;
                                _peer.set_connection(None);

                                return ();
                            }
                        }
                    }
                }
            }
            tokio::time::sleep(Duration::from_secs(15)).await;
        }
    }

    async fn reconnect(&self) {
        let (socket_, addr) = {
            loop {
                let (nns_key, nostr_client) = {
                    let _peer = self.lock().await;
                    (_peer.nns_key(), _peer.nostr_client())
                };

                match tcp::connect_nns(nns_key, &nostr_client).await {
                    Ok(socket) => {
                        let addr = match socket.peer_addr() {
                            Ok(addr) => addr,
                            Err(_) => {
                                tokio::time::sleep(Duration::from_secs(5)).await;
                                continue;
                            }
                        };

                        break (socket, addr);
                    }
                    Err(_) => {
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                }
            }
        };

        let socket: TCPSocket = Arc::new(Mutex::new(socket_));

        {
            let mut _peer = self.lock().await;
            _peer.set_connection(Some((socket, addr)));
        }
    }

    async fn set_uptimer(&self) {
        let peer = Arc::clone(&self);

        tokio::spawn(async move {
            loop {
                // Wait until disconnection.
                let (peer_kind_str, peer_addr) = {
                    let _peer = peer.lock().await;
                    (_peer.kind().as_str(), _peer.addr())
                };
                let _ = peer.disconnection().await;
                println!(
                    "{}",
                    format!(
                        "{} '{}' disconnected. Trying to connect again..",
                        peer_kind_str, peer_addr
                    )
                    .yellow()
                );

                // Re-connect upon disconnection
                let _ = peer.reconnect().await;
                let (peer_kind_str, peer_addr) = {
                    let _peer = peer.lock().await;
                    (_peer.kind().as_str(), _peer.addr())
                };
                println!(
                    "{}",
                    format!("{} '{}' re-connected.", peer_kind_str, peer_addr).green()
                );
            }
        });
    }
}

#[async_trait]
pub trait Request {
    async fn ping(&self) -> Result<Duration, RequestError>;
    async fn retrieve_vse_keymap(
        &self,
        signer_list: Vec<[u8; 32]>,
    ) -> Result<noist_vse::KeyMap, RequestError>;
}

#[derive(Copy, Clone)]
pub enum RequestError {
    TCPErr(TCPError),
    InvalidResponse,
}

#[async_trait]
impl Request for Arc<Mutex<Peer>> {
    async fn ping(&self) -> Result<Duration, RequestError> {
        // Current timestamp.
        let timestamp = Utc::now().timestamp();

        // Build request package.
        let request_package = {
            let request_kind = tcp::Kind::Ping;
            let ping_payload = [0x00]; // 0x00 for ping.
            tcp::Package::new(request_kind, timestamp, &ping_payload)
        };

        let socket: TCPSocket = {
            let _peer = self.lock().await;
            _peer
                .socket()
                .ok_or(RequestError::TCPErr(TCPError::ConnErr))?
        };

        let mut _socket = socket.lock().await;

        let timeout = Duration::from_millis(10_000);

        let (response_package, duration) =
            tcp::request(&mut *_socket, request_package, Some(timeout))
                .await
                .map_err(|err| RequestError::TCPErr(err))?;

        // Ping payload: 0x00. Pong payload: 0x01.
        let pong = [0x01];

        if &response_package.payload_bytes() == &pong {
            return Ok(duration);
        } else {
            return Err(RequestError::InvalidResponse);
        }
    }

    async fn retrieve_vse_keymap(
        &self,
        signer_list: Vec<[u8; 32]>,
    ) -> Result<noist_vse::KeyMap, RequestError> {
        // Current timestamp.
        let timestamp = Utc::now().timestamp();

        // Build request package.
        let request_package = {
            let request_kind = tcp::Kind::Ping;
            let request_payload = signer_list.encode_list();
            tcp::Package::new(request_kind, timestamp, &request_payload)
        };

        let socket: TCPSocket = {
            let _peer = self.lock().await;
            _peer
                .socket()
                .ok_or(RequestError::TCPErr(TCPError::ConnErr))?
        };

        let mut _socket = socket.lock().await;

        let timeout = Duration::from_millis(10_000);

        let (response_package, _) = tcp::request(&mut *_socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let keymap: noist_vse::KeyMap =
            match bincode::deserialize(&response_package.payload_bytes()) {
                Ok(keymap) => keymap,
                Err(_) => return Err(RequestError::InvalidResponse),
            };

        Ok(keymap)
    }
}
