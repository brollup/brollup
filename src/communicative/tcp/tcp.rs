#![allow(dead_code)]

use crate::key::ToNostrKeyStr;
use crate::{baked, nns_client};
use easy_upnp::{add_ports, PortMappingProtocol, UpnpConfig};
use std::sync::Arc;
use std::time::Duration;
use std::vec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

type NostrClient = Arc<Mutex<nostr_sdk::Client>>;
type TCPStream = Arc<Mutex<tokio::net::TcpStream>>;

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

pub async fn read(socket: &mut tokio::net::TcpStream, buffer: &mut [u8]) -> Result<(), TCPError> {
    match socket.read_exact(buffer).await {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => Err(TCPError::ConnErr),
        Err(_) => Err(TCPError::ReadErr),
    }
}

pub async fn write(socket: &mut tokio::net::TcpStream, payload: &[u8]) -> Result<(), TCPError> {
    match socket.write_all(payload).await {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => Err(TCPError::ConnErr),
        Err(_) => Err(TCPError::WriteErr),
    }
}

pub async fn request(
    socket: &mut tokio::net::TcpStream,
    requestcode: [u8; 4],
    payload: &Vec<u8>,
) -> Result<Vec<u8>, TCPError> {
    // Write requestcode.
    write(socket, &requestcode).await?;

    // Write payload len.
    let payload_length = (payload.len() as u32).to_be_bytes();
    write(socket, &payload_length).await?;

    // Write payload.
    write(socket, &payload).await?;

    // Read response length.
    let mut length_buffer = [0; 4];
    read(socket, &mut length_buffer).await?;

    // Read response.
    let response_length = u32::from_be_bytes(length_buffer) as usize;
    let mut response_payload = vec![0; response_length];
    read(socket, &mut response_payload).await?;

    return Ok(response_payload);
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
