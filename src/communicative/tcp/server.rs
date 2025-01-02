use crate::key::{KeyHolder, ToNostrKeyStr};

use crate::list::ListCodec;
use crate::schnorr::Authenticable;
use crate::tcp::Package;
use crate::{baked, nns_client, tcp, vse, OperatingMode, SIGNATORY_DB, SOCKET, VSE_DIRECTORY};
use colored::Colorize;
use std::{sync::Arc, time::Duration};
use tokio::time::Instant;
use tokio::{net::TcpListener, sync::Mutex};

pub const IDLE_CLIENT_TIMEOUT: Duration = Duration::from_secs(60);
pub const PAYLOAD_READ_TIMEOUT: Duration = Duration::from_millis(3000);
pub const PAYLOAD_WRITE_TIMEOUT: Duration = Duration::from_millis(10_000);

pub async fn run(
    mode: OperatingMode,
    nns_client: &nns_client::Client,
    keys: &KeyHolder,
    signatory_db: &SIGNATORY_DB,
    vse_directory: &VSE_DIRECTORY,
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
            let (socket_, _) = match listener.accept().await {
                Ok(conn) => (conn.0, conn.1),
                Err(_) => continue,
            };

            let socket = Arc::new(Mutex::new(socket_));

            tokio::spawn({
                let socket = Arc::clone(&socket);
                let keys = keys.clone();
                let signatory_db = Arc::clone(&signatory_db);
                let mut vse_directory = Arc::clone(&vse_directory);

                async move {
                    handle_socket(
                        &socket,
                        None,
                        mode,
                        &keys,
                        &signatory_db,
                        &mut vse_directory,
                    )
                    .await;
                }
            });
        },
        OperatingMode::Operator => {
            let coordinator_npub = match baked::COORDINATOR_WELL_KNOWN.to_npub() {
                Some(npub) => npub,
                None => return,
            };

            loop {
                match nns_client.query_address(&coordinator_npub).await {
                    Some(ip_address) => 'post_nns: loop {
                        println!("post");
                        let (socket_, socket_addr) = match listener.accept().await {
                            Ok(conn) => (conn.0, conn.1),
                            Err(_) => continue,
                        };
                        println!("ara");
                        // Operator only accepts incoming connections from the coordinator.
                        if socket_addr.ip().to_string() != ip_address {
                            continue;
                        }
                        println!("gecti");

                        let socket = Arc::new(Mutex::new(socket_));

                        let socket_alive = Arc::new(Mutex::new(true));

                        tokio::spawn({
                            let socket = Arc::clone(&socket);
                            let socket_alive = Arc::clone(&socket_alive);
                            let keys = keys.clone();
                            let signatory_db = Arc::clone(&signatory_db);
                            let mut vse_directory = Arc::clone(&vse_directory);

                            async move {
                                handle_socket(
                                    &socket,
                                    Some(&socket_alive),
                                    mode,
                                    &keys,
                                    &signatory_db,
                                    &mut vse_directory,
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
    socket: &SOCKET,
    alive: Option<&Arc<Mutex<bool>>>,
    mode: OperatingMode,
    keys: &KeyHolder,
    signatory_db: &SIGNATORY_DB,
    vse_directory: &mut VSE_DIRECTORY,
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
        handle_package(
            package,
            &mut *_socket,
            mode,
            keys,
            signatory_db,
            vse_directory,
        )
        .await;
    }

    // Remove the client from the list upon disconnection.
    {
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
    signatory_db: &SIGNATORY_DB,
    vse_directory: &mut VSE_DIRECTORY,
) {
    let response_package_ = {
        match mode {
            OperatingMode::Coordinator => match package.kind() {
                tcp::Kind::Ping => handle_ping(package.timestamp(), &package.payload()).await,
                tcp::Kind::RetrieveVSEDirectory => {
                    handle_retrieve_vse_directory(
                        package.timestamp(),
                        &package.payload(),
                        vse_directory,
                    )
                    .await
                }
                _ => return,
            },
            OperatingMode::Operator => match package.kind() {
                tcp::Kind::Ping => handle_ping(package.timestamp(), &package.payload()).await,
                tcp::Kind::RetrieveVSEKeymap => {
                    handle_retrieve_vse_keymap(package.timestamp(), &package.payload(), keys).await
                }

                tcp::Kind::DeliverVSEDirectory => {
                    handle_deliver_vse_directory(
                        package.timestamp(),
                        &package.payload(),
                        signatory_db,
                        vse_directory,
                    )
                    .await
                }

                tcp::Kind::RetrieveVSEDirectory => {
                    handle_retrieve_vse_directory(
                        package.timestamp(),
                        &package.payload(),
                        vse_directory,
                    )
                    .await
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

    let auth_keymap: Authenticable<vse::KeyMap> = Authenticable::new(keymap, keys.secret_key())?;

    let serialized: Vec<u8> = match bincode::serialize(&auth_keymap) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let package = Package::new(tcp::Kind::RetrieveVSEKeymap, timestamp, &serialized);

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

async fn handle_deliver_vse_directory(
    timestamp: i64,
    payload: &[u8],
    signatory_db: &SIGNATORY_DB,
    vse_directory: &mut VSE_DIRECTORY,
) -> Option<tcp::Package> {
    let new_directory = vse::Directory::from_slice(&payload)?;

    if !new_directory.save(signatory_db).await {
        return None;
    };

    {
        let mut _vse_directory = vse_directory.lock().await;
        *_vse_directory = new_directory;
    }

    let response_package = {
        let kind = tcp::Kind::DeliverVSEDirectory;
        let payload = [0x01u8]; // 0x01 for success.

        tcp::Package::new(kind, timestamp, &payload)
    };

    Some(response_package)
}

async fn handle_retrieve_vse_directory(
    timestamp: i64,
    payload: &[u8],
    vse_directory: &VSE_DIRECTORY,
) -> Option<tcp::Package> {
    // Expected payload: 0x00.
    if payload != &[0x00] {
        return None;
    }

    let response_package = {
        let kind = tcp::Kind::RetrieveVSEDirectory;
        let payload = {
            let _vse_directory = vse_directory.lock().await;
            _vse_directory.serialize()
        };

        tcp::Package::new(kind, timestamp, &payload)
    };

    Some(response_package)
}
