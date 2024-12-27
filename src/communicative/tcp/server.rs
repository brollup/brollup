use crate::key::{KeyHolder, ToNostrKeyStr};

use crate::list::ListCodec;
use crate::noist_vse::{self, KeyMap};
use crate::tcp::Package;
use crate::{baked, nns_query, tcp, OperatingMode};
use colored::Colorize;
use secp::{Point, Scalar};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::Instant;
use tokio::{net::TcpListener, sync::Mutex};

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;
type ClientList = Arc<Mutex<HashMap<String, TCPSocket>>>;
type NostrClient = Arc<Mutex<nostr_sdk::Client>>;

pub const IDLE_CLIENT_TIMEOUT: Duration = Duration::from_secs(60);
pub const PAYLOAD_READ_TIMEOUT: Duration = Duration::from_millis(3000);
pub const PAYLOAD_WRITE_TIMEOUT: Duration = Duration::from_millis(10_000);

pub async fn run(
    client_list: &ClientList,
    mode: OperatingMode,
    nostr_client: &NostrClient,
    keys: &KeyHolder,
) {
    let addr = format!("{}:{}", "0.0.0.0", baked::PORT);

    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(_) => {
            eprintln!("{}", format!("Failed to bind {}.", addr).red());

            return;
        }
    };

    match mode {
        OperatingMode::Coordinator => loop {
            let (socket_, socket_addr) = match listener.accept().await {
                Ok(conn) => (conn.0, conn.1),
                Err(_) => continue,
            };

            let socket = Arc::new(Mutex::new(socket_));
            let client_id = format!("{}:{}", socket_addr.ip(), socket_addr.port());

            {
                let mut _client_list = client_list.lock().await;
                _client_list.insert(client_id.clone(), Arc::clone(&socket));
            }

            tokio::spawn({
                let socket = Arc::clone(&socket);
                let client_list = Arc::clone(client_list);
                let client_id = client_id.clone();
                let keys = keys.clone();

                async move {
                    handle_socket(&socket, &client_id, &client_list, mode, &keys).await;
                }
            });
        },
        OperatingMode::Operator => {
            let coordinator_npub = match baked::COORDINATOR_WELL_KNOWN.to_npub() {
                Some(npub) => npub,
                None => return,
            };

            loop {
                match nns_query::address(&coordinator_npub, nostr_client).await {
                    Some(ip_address) => loop {
                        let (socket_, socket_addr) = match listener.accept().await {
                            Ok(conn) => (conn.0, conn.1),
                            Err(_) => continue,
                        };

                        // Operator only accepts incoming connections from the coordinator.
                        if socket_addr.ip().to_string() != ip_address {
                            continue;
                        }

                        let socket = Arc::new(Mutex::new(socket_));
                        let client_id = format!("{}:{}", socket_addr.ip(), socket_addr.port());

                        {
                            let mut _client_list = client_list.lock().await;
                            _client_list.insert(client_id.clone(), Arc::clone(&socket));
                        }

                        tokio::spawn({
                            let socket = Arc::clone(&socket);
                            let client_list = Arc::clone(client_list);
                            let client_id = client_id.clone();
                            let keys = keys.clone();

                            async move {
                                handle_socket(&socket, &client_id, &client_list, mode, &keys).await;
                            }
                        });
                    },
                    None => {
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                }
            }
        }
        OperatingMode::Node => return,
    }
}

async fn handle_socket(
    socket: &TCPSocket,
    client_id: &str,
    client_list: &ClientList,
    mode: OperatingMode,
    keys: &KeyHolder,
) {
    loop {
        let mut _socket = socket.lock().await;

        // Read package kind.
        let mut package_kind_buffer = [0; 1];
        match tcp::read(
            &mut *_socket,
            &mut package_kind_buffer,
            Some(IDLE_CLIENT_TIMEOUT),
        )
        .await
        {
            Ok(_) => (),
            Err(tcp::TCPError::ConnErr) => break, // Exit on disconnection.
            Err(tcp::TCPError::Timeout) => break, // Exit on IDLE_TIMEOUT.
            Err(_) => continue,                   // Iterate on read errors.
        }
        let package_kind = match tcp::Kind::from_bytecode(package_kind_buffer[0]) {
            Some(kind) => kind,
            None => continue,
        };

        // Start tracking elapsed time.
        let start = Instant::now();
        let timeout_duration = PAYLOAD_READ_TIMEOUT; // Default timeout: 1500 ms.

        // Read timestamp.
        let mut timestamp_buffer = [0; 8];
        match tcp::read(&mut *_socket, &mut timestamp_buffer, Some(timeout_duration)).await {
            Ok(_) => (),
            Err(tcp::TCPError::ConnErr) => break, // Exit on disconnection.
            Err(tcp::TCPError::Timeout) => continue, // Iterate on PAYLOAD_READ_TIMEOUT.
            Err(_) => continue,                   // Iterate on read errors.
        }
        let timestamp = i64::from_be_bytes(timestamp_buffer);

        let remaining_time = match timeout_duration.checked_sub(start.elapsed()) {
            Some(duration) => duration,
            None => continue,
        };

        // Read payload length.
        let mut payload_len_buffer = [0; 4];
        match tcp::read(&mut *_socket, &mut payload_len_buffer, Some(remaining_time)).await {
            Ok(_) => (),
            Err(tcp::TCPError::ConnErr) => break, // Exit on disconnection.
            Err(tcp::TCPError::Timeout) => continue, // Iterate on PAYLOAD_READ_TIMEOUT.
            Err(_) => continue,                   // Iterate on read errors.
        }

        let remaining_time = match timeout_duration.checked_sub(start.elapsed()) {
            Some(duration) => duration,
            None => continue,
        };

        // Read payload.
        let mut payload_bufer = vec![0; u32::from_be_bytes(payload_len_buffer) as usize];
        match tcp::read(&mut *_socket, &mut payload_bufer, Some(remaining_time)).await {
            Ok(_) => (),
            Err(tcp::TCPError::ConnErr) => break, // Exit on disconnection.
            Err(tcp::TCPError::Timeout) => continue, // Iterate on PAYLOAD_READ_TIMEOUT.
            Err(_) => continue,                   // Iterate on read errors.
        }

        let package = tcp::Package::new(package_kind, timestamp, &payload_bufer);

        // Process the request kind.
        handle_package(package, &mut *_socket, mode, &keys).await;
    }

    // Remove the client from the list upon disconnection.
    {
        let mut _client_list = client_list.lock().await;
        _client_list.remove(client_id);
    }
}

async fn handle_package(
    package: Package,
    socket: &mut tokio::net::TcpStream,
    mode: OperatingMode,
    keys: &KeyHolder,
) {
    match mode {
        OperatingMode::Coordinator => match package.kind() {
            tcp::Kind::Ping => {
                handle_ping(socket, package.timestamp(), &package.payload_bytes()).await
            }
            _ => return,
        },
        OperatingMode::Operator => match package.kind() {
            tcp::Kind::Ping => {
                handle_ping(socket, package.timestamp(), &package.payload_bytes()).await
            }
            tcp::Kind::RetrieveVSEKeymap => {
                handle_retrieve_vse_keymap(
                    socket,
                    package.timestamp(),
                    &package.payload_bytes(),
                    keys,
                )
                .await
            } //_ => return,
        },
        OperatingMode::Node => return,
    }
}

async fn handle_retrieve_vse_keymap(
    socket: &mut tokio::net::TcpStream,
    timestamp: i64,
    payload: &[u8],
    keys: &KeyHolder,
) {
    let signer_list = match Vec::<[u8; 32]>::decode_list(&payload.to_vec()) {
        Some(list) => list,
        None => return,
    };

    let mut keymap = KeyMap::new(keys.secret_key());

    for signer in signer_list.iter() {
        let self_secret = match Scalar::from_slice(&keys.secret_key()) {
            Ok(scalar) => scalar,
            Err(_) => return,
        };

        let to_public = match Point::from_slice(signer) {
            Ok(point) => point,
            Err(_) => return,
        };

        let vse_key = noist_vse::encrypting_key_public(self_secret, to_public).serialize_xonly();

        keymap.insert(signer.to_owned(), vse_key);
    }

    if !keymap.is_complete(&signer_list) {
        return;
    }

    let serialized_keymap: Vec<u8> = match bincode::serialize(&keymap) {
        Ok(bytes) => bytes,
        Err(_) => return,
    };

    let response_payload = Package::new(tcp::Kind::Ping, timestamp, &serialized_keymap).serialize();

    if let Err(_) = tcp::write(socket, &response_payload, Some(PAYLOAD_WRITE_TIMEOUT)).await {
        return;
    }
}

async fn handle_ping(socket: &mut tokio::net::TcpStream, timestamp: i64, payload: &[u8]) {
    // Ping payload: 0x00. Pong payload: 0x01.
    let pong_payload = [0x01];

    let response_payload = Package::new(tcp::Kind::Ping, timestamp, &pong_payload).serialize();

    // Ping payload: 0x00. Pong payload: 0x01.
    if payload == &[0x00] {
        if let Err(_) = tcp::write(socket, &response_payload, Some(PAYLOAD_WRITE_TIMEOUT)).await {
            return;
        }
    } else {
        return;
    }
}
