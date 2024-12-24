#![allow(dead_code)]

use crate::key::ToNostrKeyStr;
use crate::{baked, nns_client};
use easy_upnp::{add_ports, PortMappingProtocol, UpnpConfig};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::vec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

type NostrClient = Arc<Mutex<nostr_sdk::Client>>;
type TCPStream = Arc<Mutex<tokio::net::TcpStream>>;

pub const IDLE_TIMEOUT: Duration = Duration::from_secs(3600);
pub const REQUEST_TIMEOUT: Duration = Duration::from_millis(1500);

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

#[derive(Debug, Copy, Clone)]
pub enum TCPError {
    ConnErr,
    ReadErr,
    WriteErr,
    Timeout,
}

pub async fn open_port() -> bool {
    let upnp_config = UpnpConfig {
        address: None,
        port: baked::PORT,
        protocol: PortMappingProtocol::TCP,
        duration: 100_000_000,
        comment: format!("{} {}", baked::PROJECT_TAG, "Transport Layer"),
    };

    for result in add_ports([upnp_config]) {
        if let Ok(_) = result {
            return true;
        }
    }

    false
}

pub async fn connect(ip_address: &str) -> Result<TCPStream, TCPError> {
    let addr = format!("{}:{}", ip_address, baked::PORT);

    let conn = tokio::time::timeout(
        Duration::from_secs(baked::TCP_RESPONSE_TIMEOUT),
        TcpStream::connect(addr),
    )
    .await;

    match conn {
        Ok(Ok(stream)) => {
            let stream = Arc::new(Mutex::new(stream));

            Ok(stream)
        }
        _ => Err(TCPError::ConnErr),
    }
}

pub async fn connect_nns(
    public_key: [u8; 32],
    nostr_client: &NostrClient,
) -> Result<TCPStream, TCPError> {
    let npub = match public_key.to_npub() {
        Some(npub) => npub,
        None => return Err(TCPError::ConnErr),
    };

    let ip_address = nns_client::retrieve_ip_address(&npub, nostr_client)
        .await
        .unwrap();

    connect(&ip_address).await
}

pub async fn read(
    socket: &mut tokio::net::TcpStream,
    buffer: &mut [u8],
    timeout: Option<Duration>,
) -> Result<(), TCPError> {
    let result = match timeout {
        Some(duration) => tokio::time::timeout(duration, socket.read_exact(buffer))
            .await
            .map_err(|_| TCPError::Timeout)?,
        None => socket.read_exact(buffer).await,
    };

    match result {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => Err(TCPError::ConnErr),
        Err(_) => Err(TCPError::ReadErr),
    }
}

pub async fn write(
    socket: &mut tokio::net::TcpStream,
    payload: &[u8],
    timeout: Option<Duration>,
) -> Result<(), TCPError> {
    let result = match timeout {
        Some(duration) => tokio::time::timeout(duration, socket.write_all(payload))
            .await
            .map_err(|_| TCPError::Timeout)?,
        None => socket.write_all(payload).await,
    };

    match result {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => Err(TCPError::ConnErr),
        Err(_) => Err(TCPError::WriteErr),
    }
}

pub async fn request(
    socket: &mut tokio::net::TcpStream,
    requestcode: [u8; 4],
    payload: &[u8],
    timeout: Option<Duration>,
) -> Result<Vec<u8>, TCPError> {
    // Determine the timeout duration.
    let timeout = timeout.unwrap_or(REQUEST_TIMEOUT); // Default timeout: 1500 ms.

    // Build the request buffer.
    let mut request_buffer = Vec::with_capacity(4 + 4 + payload.len());
    request_buffer.extend_from_slice(&requestcode); // Add requestcode; 4 bytes.
    request_buffer.extend_from_slice(&(payload.len() as u32).to_be_bytes()); // Add payload length; 4 bytes.
    request_buffer.extend_from_slice(payload); // Add payload; variable-length size bytes.

    // Start tracking elapsed time.
    let start = Instant::now();

    // Write the request buffer with timeout.
    let remaining_time = timeout
        .checked_sub(start.elapsed())
        .ok_or(TCPError::Timeout)?;

    write(socket, &request_buffer, Some(remaining_time)).await?;

    // Read the response length; 4 bytes.
    let mut length_buffer = [0; 4];
    let remaining_time = timeout
        .checked_sub(start.elapsed())
        .ok_or(TCPError::Timeout)?;

    read(socket, &mut length_buffer, Some(remaining_time)).await?;

    // Read the response; variable-length bytes.
    let response_length = u32::from_be_bytes(length_buffer) as usize;
    let mut response_buffer = vec![0; response_length];
    let remaining_time = timeout
        .checked_sub(start.elapsed())
        .ok_or(TCPError::Timeout)?;

    read(socket, &mut response_buffer, Some(remaining_time)).await?;

    Ok(response_buffer)
}

pub async fn connectivity() -> bool {
    match tokio::time::timeout(
        Duration::from_secs(baked::TCP_RESPONSE_TIMEOUT),
        TcpStream::connect("8.8.8.8:53"),
    )
    .await
    {
        Ok(Ok(_stream)) => true,
        _ => false,
    }
}
