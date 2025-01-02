use crate::tcp::{self, TCPError};
use crate::tcp_client::Client;
use crate::{nns_client, PEER, PEER_LIST, SOCKET};
use async_trait::async_trait;
use colored::Colorize;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::Mutex;

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
    nns_client: nns_client::Client,
    connection: Option<(SOCKET, SocketAddr)>,
}

impl Peer {
    pub async fn connect(
        kind: PeerKind,
        nns_key: [u8; 32],
        nns_client: &nns_client::Client,
    ) -> Result<Arc<Mutex<Self>>, TCPError> {
        let (socket_, addr) = {
            match tcp::connect_nns(nns_key, &nns_client).await {
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

        let socket: SOCKET = Arc::new(Mutex::new(socket_));

        let connection = Some((socket, addr));

        let peer_ = Peer {
            kind,
            nns_key,
            connection,
            nns_client: nns_client.clone(),
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

    pub fn nns_client(&self) -> nns_client::Client {
        self.nns_client.clone()
    }

    pub fn connection(&self) -> Option<(SOCKET, SocketAddr)> {
        self.connection.clone()
    }

    pub fn socket(&self) -> Option<SOCKET> {
        let socket = Arc::clone(&self.connection()?.0);
        Some(socket)
    }

    pub fn set_connection(&mut self, connection: Option<(SOCKET, SocketAddr)>) {
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
    async fn socket(&self) -> Option<SOCKET>;
    async fn disconnection(&self);
    async fn reconnect(&self);
    async fn set_uptimer(&self);
}

#[async_trait]
impl Connection for Arc<Mutex<Peer>> {
    async fn socket(&self) -> Option<SOCKET> {
        let _self = self.lock().await;
        _self.socket()
    }

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
                let (nns_key, nns_client) = {
                    let _peer = self.lock().await;
                    (_peer.nns_key(), _peer.nns_client())
                };

                match tcp::connect_nns(nns_key, &nns_client).await {
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

        let socket: SOCKET = Arc::new(Mutex::new(socket_));

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

#[async_trait::async_trait]
pub trait PeerListExt {
    async fn active_peers(&self) -> Vec<PEER>;
    async fn active_keys(&self) -> Vec<[u8; 32]>;
}

#[async_trait::async_trait]
impl PeerListExt for PEER_LIST {
    async fn active_peers(&self) -> Vec<PEER> {
        let mut list = Vec::<PEER>::new();

        let _operator_list = self.lock().await;

        for (_, peer) in _operator_list.iter().enumerate() {
            let conn = {
                let _peer = peer.lock().await;
                _peer.connection()
            };

            if let Some(_) = conn {
                {
                    let _peer = peer.lock().await;
                    //connected_operator_key_list.push(_peer.nns_key());
                }

                list.push(Arc::clone(&peer));
            }
        }
        list
    }

    async fn active_keys(&self) -> Vec<[u8; 32]> {
        let mut key_list = Vec::<[u8; 32]>::new();

        for peer in self.active_peers().await.iter() {
            let _peer = peer.lock().await;
            key_list.push(_peer.nns_key());
        }

        key_list
    }
}
