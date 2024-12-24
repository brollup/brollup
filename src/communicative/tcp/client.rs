use crate::{
    tcp::{self, TCPError},
    PeerKind,
};
use futures::future::join_all;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;
type SocketList = Arc<Mutex<HashMap<String, (TCPSocket, PeerKind)>>>;

#[derive(Copy, Clone)]
pub enum ClientError {
    TCPErr(TCPError),
    InvalidResponse,
}

pub async fn uptime_peer_list(peer_list: &SocketList) {
    loop {
        let peers = {
            let _peer_list = peer_list.lock().await;
            _peer_list.clone()
        };

        let mut removal_list = Vec::<String>::new();

        // Collect async tasks to wait on
        let mut tasks = Vec::new();

        for peer in peers.iter() {
            let peer_id = peer.0.clone();
            let peer_socket = peer.1 .0.clone();

            let task = async move {
                let mut failure_iter: u8 = 0;

                loop {
                    match ping(&peer_socket).await {
                        Ok(_) => break,
                        Err(_) => {
                            if failure_iter < 3 {
                                failure_iter += 1;
                                tokio::time::sleep(Duration::from_secs(5)).await;
                                continue;
                            } else {
                                return Some(peer_id);
                            }
                        }
                    }
                }
                None
            };

            tasks.push(tokio::spawn(task));
        }

        // Wait until all async tasks complete
        let results = join_all(tasks).await;

        // Collect removal IDs from results
        for result in results {
            if let Ok(Some(peer_id)) = result {
                removal_list.push(peer_id);
            }
        }

        // Remove peers that failed
        {
            let mut _peer_list = peer_list.lock().await;
            for peer_id in removal_list.iter() {
                _peer_list.remove(peer_id);
            }
        }

        tokio::time::sleep(Duration::from_secs(15)).await;
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
