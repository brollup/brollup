#![allow(dead_code)]

use crate::key::ToNostrKeyStr;
use crate::{baked, nns_client};
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
    pub fn from_bytecode(bytecode: u8) -> Option<Self> {
        let request_kind = match bytecode {
            0x00 => RequestKind::Ping,
            _ => return None,
        };

        Some(request_kind)
    }
}

pub async fn check_connectivity() -> bool {
    match tokio::time::timeout(
        Duration::from_secs(baked::TCP_RESPONSE_TIMEOUT),
        TcpStream::connect("8.8.8.8:53"),
    )
    .await
    {
        Ok(Ok(_stream)) => true,
        Ok(Err(_)) => false,
        Err(_) => false,
    }
}
pub enum TCPError {
    ConnectErr,
    WriteErr,
    ReadErr,
}

pub async fn connect_nns(public_key: [u8; 32], nostr_client: &NostrClient) -> Option<TCPStream> {
    let npub = match public_key.to_npub() {
        Some(npub) => npub,
        None => return None,
    };

    let ip_address = nns_client::retrieve_ip_address(&npub, nostr_client)
        .await
        .unwrap();
    println!("trying to connect: {}", ip_address);
    connect(&ip_address).await
}

pub async fn connect(ip_address: &str) -> Option<TCPStream> {
    let conn = tokio::time::timeout(
        Duration::from_secs(baked::TCP_RESPONSE_TIMEOUT),
        TcpStream::connect(ip_address.to_string() + ":" + &baked::PORT.to_string()),
    )
    .await;

    match conn {
        Ok(Ok(stream)) => {
            let stream = Arc::new(Mutex::new(stream));

            Some(stream)
        }
        _ => None,
    }
}

pub async fn request(
    stream: &TCPStream,
    request_bytecode: u8,
    payload: &Vec<u8>,
) -> Result<Vec<u8>, TCPError> {
    let mut stream_ = stream.lock().await;

    // Write request bytecode.
    match stream_.write_all(&[request_bytecode]).await {
        Ok(_stream) => (),
        Err(_) => return Err(TCPError::WriteErr),
    }

    // Write payload len.
    let payload_len = payload.len() as u32;
    match stream_.write_all(&payload_len.to_be_bytes()).await {
        Ok(_stream) => (),
        Err(_) => return Err(TCPError::WriteErr),
    }

    // Write payload.
    match stream_.write_all(payload).await {
        Ok(_stream) => (),
        Err(_) => return Err(TCPError::WriteErr),
    }

    // Read response length.
    let mut length_buffer = [0; 4];
    match stream_.read_exact(&mut length_buffer).await {
        Ok(_stream) => (),
        Err(_) => return Err(TCPError::ReadErr),
    }

    // Read response.
    let response_length = u32::from_be_bytes(length_buffer) as usize;
    let mut response_payload = vec![0; response_length];
    match stream_.read_exact(&mut response_payload).await {
        Ok(_stream) => (),
        Err(_) => return Err(TCPError::ReadErr),
    }

    return Ok(response_payload);
}
