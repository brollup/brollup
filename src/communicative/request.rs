use crate::tcp::{self, TCPError};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;

#[derive(Copy, Clone)]
pub enum RequestKind {
    Ping,
}

impl RequestKind {
    pub fn bytecode(&self) -> u8 {
        match self {
            RequestKind::Ping => 0x00,
        }
    }
    pub fn to_requestcode(&self) -> [u8; 4] {
        // Requestcode stars with 'b' 'r' 'l'.
        [0x62, 0x72, 0x6c, self.bytecode()]
    }
    pub fn from_requestcode(requestcode: [u8; 4]) -> Option<Self> {
        let brl = &requestcode[..3];

        if brl != vec![0x62, 0x72, 0x6c] {
            return None;
        } else {
            match &requestcode[3] {
                0x00 => return Some(RequestKind::Ping),
                _ => return None,
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum RequestError {
    TCPErr(TCPError),
    InvalidResponse,
}

/// Pings a peer.
///
pub async fn ping(socket: &TCPSocket) -> Result<(), RequestError> {
    let requestcode = RequestKind::Ping.to_requestcode();
    let request_payload = [0x00];

    let mut _socket = socket.lock().await;

    let timeout = Duration::from_millis(3000);

    let response = tcp::request(&mut *_socket, requestcode, &request_payload, Some(timeout))
        .await
        .map_err(|err| RequestError::TCPErr(err))?;

    let pong = RequestKind::Ping.to_requestcode();

    if &response == &pong {
        return Ok(());
    } else {
        return Err(RequestError::InvalidResponse);
    }
}
