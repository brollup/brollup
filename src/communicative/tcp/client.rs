use crate::{
    baked,
    tcp::{self, TCPError},
    PeerKind,
};
use colored::Colorize;
use futures::future::join_all;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::Mutex;

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;
type Connection = Arc<Mutex<Option<TCPSocket>>>;
type NostrClient = Arc<Mutex<nostr_sdk::Client>>;

#[derive(Clone)]
pub struct Peer {
    nns_key: [u8; 32],
    nostr_client: NostrClient,
    connection: Option<(TCPSocket, SocketAddr)>,
}

impl Peer {
    pub async fn connect(
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
            nns_key,
            connection,
            nostr_client: Arc::clone(nostr_client),
        };

        let peer = Arc::new(Mutex::new(peer_));

        Ok(peer)
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

    pub async fn conn(&self) {
        match self.connection() {
            Some(_) => {
                let addr: String = self.addr();
                println!("Alive: {}", addr);
            }
            None => {
                println!("Dead.")
            }
        }
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

#[derive(Copy, Clone)]
pub enum ClientError {
    TCPErr(TCPError),
    InvalidResponse,
}

async fn try_reconnect(peer: &Arc<Mutex<Peer>>) -> () {
    let (socket_, addr) = {
        loop {
            let (nns_key, nostr_client) = {
                let _peer = peer.lock().await;
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
        let mut _peer = peer.lock().await;
        _peer.set_connection(Some((socket, addr)));
    }

    println!("reconnected.");

    ()
}

async fn disconnection(peer: &Arc<Mutex<Peer>>) -> () {
    let socket = {
        let _peer = peer.lock().await;

        match _peer.socket() {
            Some(socket) => socket,
            None => return (),
        }
    };

    loop {
        loop {
            match ping(&socket).await {
                Ok(_) => break,
                Err(_) => {
                    let mut failure_iter: u8 = 0;
                    loop {
                        if failure_iter < 3 {
                            failure_iter += 1;
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            continue;
                        } else {
                            let mut _peer = peer.lock().await;
                            _peer.set_connection(None);

                            return ();
                        }
                    }
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

pub async fn uptime(peer: &Arc<Mutex<Peer>>) {
    loop {
        // Wait until disconnection.
        let peer_addr = {
            let _peer = peer.lock().await;
            _peer.addr()
        };
        let _ = disconnection(&peer).await;
        println!("{}", format!("Disconnected: {}", peer_addr).red());

        // Re-connect upon disconnection
        let _ = try_reconnect(&peer).await;
        let peer_addr = {
            let _peer = peer.lock().await;
            _peer.addr()
        };
        println!("{}", format!("Re-connected: {}", peer_addr).green());
    }
}

/// Pings a peer.
///
pub async fn ping(socket: &TCPSocket) -> Result<(), ClientError> {
    let requestcode = tcp::RequestKind::Ping.to_requestcode();
    let request_payload = [0x00];

    let mut _socket = socket.lock().await;

    let response = tcp::request(
        &mut *_socket,
        requestcode,
        &request_payload,
        Some(tcp::REQUEST_TIMEOUT),
    )
    .await
    .map_err(|err| ClientError::TCPErr(err))?;

    let pong = tcp::RequestKind::Ping.to_requestcode();

    if &response == &pong {
        return Ok(());
    } else {
        return Err(ClientError::InvalidResponse);
    }
}
