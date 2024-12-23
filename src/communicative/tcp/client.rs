use crate::{baked, tcp};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

type TCPStream = Arc<Mutex<tokio::net::TcpStream>>;

#[derive(Copy, Clone)]
pub enum ClientError {
    Timeout,
    InvalidResponse,
    EmptyResponse,
    NoResponse,
}

/// Pings a peer.
///
pub async fn ping(socket: &TCPStream) -> Result<(), ClientError> {
    let requestcode = tcp::RequestKind::Ping.to_requestcode();
    let request_payload = Vec::<u8>::with_capacity(0);

    let mut _socket = socket.lock().await;

    let response = tokio::time::timeout(
        Duration::from_secs(baked::TCP_RESPONSE_TIMEOUT),
        tcp::request(&mut *_socket, requestcode, &request_payload),
    )
    .await;

    match response {
        Ok(response) => match response {
            Ok(response) => {
                if &response == &[tcp::RequestKind::Ping.bytecode()] {
                    return Ok(());
                } else {
                    return Err(ClientError::InvalidResponse);
                }
            }
            Err(_) => {
                return Err(ClientError::NoResponse);
            }
        },
        Err(_) => {
            return Err(ClientError::Timeout);
        }
    }
}
