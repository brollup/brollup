use crate::{
    tcp::{self, TCPError},
    tcp_request,
};
use async_trait::async_trait;
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
                return "".to_string();
            }
        };
    }
}

#[async_trait]
pub trait PeerConnection {
    async fn disconnection(&self);
    async fn try_reconnect(&self);
    async fn set_uptimer(&self);
}

#[async_trait]
impl PeerConnection for Arc<Mutex<Peer>> {
    async fn disconnection(&self) {
        let socket = {
            let _peer = self.lock().await;

            match _peer.socket() {
                Some(socket) => socket,
                None => return,
            }
        };

        loop {
            loop {
                match tcp_request::ping(&socket).await {
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

    async fn try_reconnect(&self) {
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
                let _ = peer.try_reconnect().await;
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
