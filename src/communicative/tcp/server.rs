use crate::key::{KeyHolder, ToNostrKeyStr};

use crate::list::ListCodec;
use crate::tcp::Package;
use crate::{baked, nns_query, tcp, vse, OperatingMode};
use colored::Colorize;
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
                    handle_socket(&socket, None, &client_id, &client_list, mode, &keys).await;
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
                    Some(ip_address) => 'post_nns: loop {
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

                        let socket_alive = Arc::new(Mutex::new(true));

                        tokio::spawn({
                            let socket = Arc::clone(&socket);
                            let socket_alive = Arc::clone(&socket_alive);
                            let client_list = Arc::clone(client_list);
                            let client_id = client_id.clone();
                            let keys = keys.clone();

                            async move {
                                handle_socket(
                                    &socket,
                                    Some(&socket_alive),
                                    &client_id,
                                    &client_list,
                                    mode,
                                    &keys,
                                )
                                .await;
                            }
                        });

                        loop {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            let alive = {
                                let mut _alive = socket_alive.lock().await;
                                *_alive
                            };

                            if !alive {
                                break 'post_nns;
                            }
                        }
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
    alive: Option<&Arc<Mutex<bool>>>,
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
        let timeout_duration = PAYLOAD_READ_TIMEOUT; // Default timeout: 3000 ms.

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
        let payload_len = u32::from_be_bytes(payload_len_buffer) as usize;

        let remaining_time = match timeout_duration.checked_sub(start.elapsed()) {
            Some(duration) => duration,
            None => continue,
        };

        // Read payload.
        let mut payload_bufer = vec![0x00u8; u32::from_be_bytes(payload_len_buffer) as usize];
        match payload_len {
            0 => continue, // Iterate on empty payload.
            _ => {
                match tcp::read(&mut *_socket, &mut payload_bufer, Some(remaining_time)).await {
                    Ok(_) => (),
                    Err(tcp::TCPError::ConnErr) => break, // Exit on disconnection.
                    Err(tcp::TCPError::Timeout) => continue, // Iterate on PAYLOAD_READ_TIMEOUT.
                    Err(_) => continue,                   // Iterate on read errors.
                }
            }
        }

        let package = tcp::Package::new(package_kind, timestamp, &payload_bufer);

        // Process the request kind.
        handle_package(package, &mut *_socket, mode, &keys).await;
    }

    // Remove the client from the list upon disconnection.
    {
        let mut _client_list = client_list.lock().await;
        _client_list.remove(client_id);

        if let Some(alive) = alive {
            let mut _alive = alive.lock().await;
            *_alive = false;
        }
    }
}

async fn handle_package(
    package: Package,
    socket: &mut tokio::net::TcpStream,
    mode: OperatingMode,
    keys: &KeyHolder,
) {
    let response_package_ = {
        match mode {
            OperatingMode::Coordinator => match package.kind() {
                tcp::Kind::Ping => handle_ping(package.timestamp(), &package.payload()).await,
                _ => return,
            },
            OperatingMode::Operator => match package.kind() {
                tcp::Kind::Ping => handle_ping(package.timestamp(), &package.payload()).await,
                tcp::Kind::RetrieveVSEKeymap => {
                    handle_retrieve_vse_keymap(package.timestamp(), &package.payload(), keys).await
                } //_ => return,
            },
            OperatingMode::Node => return,
        }
    };

    let response_package = match response_package_ {
        Some(package) => package,
        // Empty package if None.
        None => tcp::Package::new(package.kind(), package.timestamp(), &[]),
    };

    let _ = response_package
        .deliver(socket, Some(PAYLOAD_WRITE_TIMEOUT))
        .await;
}

async fn handle_retrieve_vse_keymap(
    timestamp: i64,
    payload: &[u8],
    keys: &KeyHolder,
) -> Option<tcp::Package> {
    let mut signer_list = match Vec::<[u8; 32]>::decode_list(&payload.to_vec()) {
        Some(list) => list,
        None => return None,
    };
    signer_list.sort();

    // # Security block.
    {
        for signer in signer_list.iter() {
            if !baked::OPERATOR_SET.contains(signer) {
                return None;
            }
        }
        // TODO: Add majority check.
    }

    let mut keymap = vse::KeyMap::new(keys.public_key());

    if !keymap.fill(keys.secret_key(), &signer_list) {
        return None;
    };

    if !keymap.is_complete(&signer_list) {
        return None;
    }

    let serialized_keymap: Vec<u8> = match bincode::serialize(&keymap) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let package = Package::new(tcp::Kind::RetrieveVSEKeymap, timestamp, &serialized_keymap);

    Some(package)
}

async fn handle_ping(timestamp: i64, payload: &[u8]) -> Option<tcp::Package> {
    // Expected payload: 0x00 ping.
    if payload != &[0x00] {
        return None;
    }

    let response_package = {
        let kind = tcp::Kind::Ping;
        let payload = [0x01u8]; // 0x01 for pong.

        tcp::Package::new(kind, timestamp, &payload)
    };

    Some(response_package)
}
