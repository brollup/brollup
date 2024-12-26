use crate::tcp::{self, TCPError};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;

#[derive(Copy, Clone)]
pub enum RequestError {
    TCPErr(TCPError),
    InvalidResponse,
}

/// Pings a peer.
///
pub async fn ping(socket: &TCPSocket) -> Result<Duration, RequestError> {
    let request_kind = tcp::Kind::Ping;

    // Ping payload: 0x00. Pong payload: 0x01.
    let request_payload = [0x00];

    let request_package = tcp::Package::new(request_kind, &request_payload);

    let mut _socket = socket.lock().await;

    let timeout = Duration::from_millis(10_000);

    let (response_package, duration) = tcp::request(&mut *_socket, request_package, Some(timeout))
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
