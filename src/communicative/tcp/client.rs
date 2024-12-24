use crate::tcp::{self, TCPError};
use std::sync::Arc;
use tokio::sync::Mutex;

type TCPStream = Arc<Mutex<tokio::net::TcpStream>>;

#[derive(Copy, Clone)]
pub enum ClientError {
    TCPErr(TCPError),
    InvalidResponse,
}

/// Pings a peer.
///
pub async fn ping(socket: &TCPStream) -> Result<(), ClientError> {
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
