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
pub async fn ping(stream: &TCPStream) -> Result<(), ClientError> {
    let request_bytecode = tcp::RequestKind::Ping.bytecode();
    let request_payload = Vec::<u8>::with_capacity(0);

    let response = tokio::time::timeout(
        Duration::from_secs(baked::TCP_RESPONSE_TIMEOUT),
        tcp::request(stream, request_bytecode, &request_payload),
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
