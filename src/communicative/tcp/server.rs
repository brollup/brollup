use super::package::{PackageKind, TCPPackage};
use super::tcp;
use crate::into::IntoPointVec;
use crate::key::{KeyHolder, ToNostrKeyStr};
use crate::list::ListCodec;
use crate::nns::client::NNSClient;
use crate::noist::setup::{keymap::VSEKeyMap, setup::VSESetup};
use crate::schnorr::Authenticable;

use crate::{baked, OperatingMode, NOIST_MANAGER, SOCKET};
use colored::Colorize;
use std::{sync::Arc, time::Duration};
use tokio::time::Instant;
use tokio::{net::TcpListener, sync::Mutex};

pub const IDLE_CLIENT_TIMEOUT: Duration = Duration::from_secs(60);
pub const PAYLOAD_READ_TIMEOUT: Duration = Duration::from_millis(3000);
pub const PAYLOAD_WRITE_TIMEOUT: Duration = Duration::from_millis(10_000);

pub async fn run(
    mode: OperatingMode,
    nns_client: &NNSClient,
    keys: &KeyHolder,
    noist_manager: &NOIST_MANAGER,
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
                let mut noist_manager = Arc::clone(&noist_manager);

                async move {
                    handle_socket(&socket, None, mode, &keys, &mut noist_manager).await;
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
                        let (socket_, socket_addr) = match listener.accept().await {
                            Ok(conn) => (conn.0, conn.1),
                            Err(_) => continue,
                        };

                        // Operator only accepts incoming connections from the coordinator.
                        if socket_addr.ip().to_string() != ip_address {
                            continue;
                        }

                        let socket = Arc::new(Mutex::new(socket_));

                        let socket_alive = Arc::new(Mutex::new(true));

                        tokio::spawn({
                            let socket = Arc::clone(&socket);
                            let socket_alive = Arc::clone(&socket_alive);
                            let keys = keys.clone();
                            let mut noist_manager = Arc::clone(&noist_manager);

                            async move {
                                handle_socket(
                                    &socket,
                                    Some(&socket_alive),
                                    mode,
                                    &keys,
                                    &mut noist_manager,
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
    noist_manager: &mut NOIST_MANAGER,
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
        let package_kind = match PackageKind::from_bytecode(package_kind_buffer[0]) {
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

        let package = TCPPackage::new(package_kind, timestamp, &payload_bufer);

        // Process the request kind.
        handle_package(package, &mut *_socket, mode, keys, noist_manager).await;
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
    package: TCPPackage,
    socket: &mut tokio::net::TcpStream,
    mode: OperatingMode,
    keys: &KeyHolder,
    noist_manager: &mut NOIST_MANAGER,
) {
    let response_package_ = {
        match mode {
            OperatingMode::Coordinator => match package.kind() {
                PackageKind::Ping => handle_ping(package.timestamp(), &package.payload()).await,
                PackageKind::RetrieveVSESetup => {
                    handle_retrieve_vse_setup(
                        package.timestamp(),
                        &package.payload(),
                        noist_manager,
                    )
                    .await
                }
                _ => return,
            },
            OperatingMode::Operator => match package.kind() {
                PackageKind::Ping => handle_ping(package.timestamp(), &package.payload()).await,
                PackageKind::RequestVSEKeymap => {
                    handle_request_vse_keymap(package.timestamp(), &package.payload(), keys).await
                }

                PackageKind::DeliverVSESetup => {
                    handle_deliver_vse_setup(package.timestamp(), &package.payload(), noist_manager)
                        .await
                }

                PackageKind::RetrieveVSESetup => {
                    handle_retrieve_vse_setup(
                        package.timestamp(),
                        &package.payload(),
                        noist_manager,
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
        None => TCPPackage::new(package.kind(), package.timestamp(), &[]),
    };

    let _ = response_package
        .deliver(socket, Some(PAYLOAD_WRITE_TIMEOUT))
        .await;
}

async fn handle_request_vse_keymap(
    timestamp: i64,
    payload: &[u8],
    keys: &KeyHolder,
) -> Option<TCPPackage> {
    let mut signer_list = match Vec::<[u8; 32]>::decode_list(&payload.to_vec()) {
        Some(list) => list,
        None => return None,
    };
    signer_list.sort();

    let signer_list = signer_list.into_point_vec().ok()?;

    // # Security block.
    {
        for signer in signer_list.iter() {
            if !baked::OPERATOR_SET.contains(&signer.serialize_xonly()) {
                return None;
            }
        }
        // TODO: Add majority check.
    }

    let keymap = VSEKeyMap::new(keys.secret_key(), &signer_list)?;

    let auth_keymap: Authenticable<VSEKeyMap> = Authenticable::new(keymap, keys.secret_key())?;

    let serialized: Vec<u8> = match bincode::serialize(&auth_keymap) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let package = TCPPackage::new(PackageKind::RequestVSEKeymap, timestamp, &serialized);

    Some(package)
}

async fn handle_ping(timestamp: i64, payload: &[u8]) -> Option<TCPPackage> {
    // Expected payload: 0x00 ping.
    if payload != &[0x00] {
        return None;
    }

    let response_package = {
        let kind = PackageKind::Ping;
        let payload = [0x01u8]; // 0x01 for pong.

        TCPPackage::new(kind, timestamp, &payload)
    };

    Some(response_package)
}

async fn handle_deliver_vse_setup(
    timestamp: i64,
    payload: &[u8],
    noist_manager: &mut NOIST_MANAGER,
) -> Option<TCPPackage> {
    let vse_setup = VSESetup::from_slice(&payload)?;

    let insertion = {
        let mut _noist_manager = noist_manager.lock().await;
        _noist_manager.insert_setup(&vse_setup)
    };

    let response_package = {
        let kind = PackageKind::DeliverVSESetup;
        let payload = match insertion {
            true => [0x01u8],  // 0x00 for success.
            false => [0x00u8], // 0x01 for failure.
        };

        TCPPackage::new(kind, timestamp, &payload)
    };

    Some(response_package)
}

async fn handle_retrieve_vse_setup(
    timestamp: i64,
    payload: &[u8],
    noist_manager: &NOIST_MANAGER,
) -> Option<TCPPackage> {
    let setup_no_bytes: [u8; 8] = match payload.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let setup_no = u64::from_be_bytes(setup_no_bytes);

    let vse_setup = {
        let _noist_manager = noist_manager.lock().await;
        match _noist_manager.directory(setup_no) {
            Some(dir) => dir.setup().clone(),
            None => return None,
        }
    };

    let response_package = {
        let kind = PackageKind::RetrieveVSESetup;
        let payload = vse_setup.serialize();

        TCPPackage::new(kind, timestamp, &payload)
    };

    Some(response_package)
}
