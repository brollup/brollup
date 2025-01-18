use super::package::{PackageKind, TCPPackage};
use super::tcp;
use crate::into::IntoPointVec;
use crate::key::{KeyHolder, ToNostrKeyStr};
use crate::nns::client::NNSClient;
use crate::noist::dkg::package::DKGPackage;
use crate::noist::dkg::session::DKGSession;
use crate::noist::setup::{keymap::VSEKeyMap, setup::VSESetup};
use crate::schnorr::Authenticable;
use crate::{baked, liquidity, OperatingMode, DKG_DIRECTORY, DKG_MANAGER, SOCKET};
use colored::Colorize;
use secp::Scalar;
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
    dkg_manager: &DKG_MANAGER,
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
                let mut dkg_manager = Arc::clone(&dkg_manager);

                async move {
                    handle_socket(&socket, None, mode, &keys, &mut dkg_manager).await;
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
                            let mut dkg_manager = Arc::clone(&dkg_manager);

                            async move {
                                handle_socket(
                                    &socket,
                                    Some(&socket_alive),
                                    mode,
                                    &keys,
                                    &mut dkg_manager,
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
    dkg_manager: &mut DKG_MANAGER,
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
        handle_package(package, &mut *_socket, mode, keys, dkg_manager).await;
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
    dkg_manager: &mut DKG_MANAGER,
) {
    let response_package_ = {
        match mode {
            OperatingMode::Coordinator => match package.kind() {
                PackageKind::Ping => handle_ping(package.timestamp(), &package.payload()).await,
                PackageKind::RetrieveVSESetup => {
                    handle_retrieve_vse_setup(package.timestamp(), &package.payload(), dkg_manager)
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
                    handle_deliver_vse_setup(package.timestamp(), &package.payload(), dkg_manager)
                        .await
                }

                PackageKind::RetrieveVSESetup => {
                    handle_retrieve_vse_setup(package.timestamp(), &package.payload(), dkg_manager)
                        .await
                }

                PackageKind::RequestDKGPackages => {
                    handle_request_dkg_packages(
                        package.timestamp(),
                        &package.payload(),
                        dkg_manager,
                        keys,
                    )
                    .await
                }

                PackageKind::DeliverDKGSessions => {
                    handle_deliver_dkg_sessions(
                        package.timestamp(),
                        &package.payload(),
                        dkg_manager,
                    )
                    .await
                }

                PackageKind::RequestPartialSigs => {
                    handle_request_partial_sigs(
                        package.timestamp(),
                        &package.payload(),
                        dkg_manager,
                        keys,
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
    let signatory_keys: Vec<[u8; 32]> = match serde_json::from_slice(payload) {
        Ok(no) => no,
        Err(_) => return None,
    };

    if !liquidity::provider::is_valid_subset(&signatory_keys) {
        return None;
    }

    let signatories = signatory_keys.into_point_vec().ok()?;

    let keymap = VSEKeyMap::new(keys.secret_key(), &signatories)?;

    let package = TCPPackage::new(
        PackageKind::RequestVSEKeymap,
        timestamp,
        &keymap.serialize(),
    );

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
    dkg_manager: &mut DKG_MANAGER,
) -> Option<TCPPackage> {
    let vse_setup = VSESetup::from_slice(&payload)?;

    let insertion = {
        let mut _dkg_manager = dkg_manager.lock().await;
        _dkg_manager.insert_setup(&vse_setup)
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
    dkg_manager: &DKG_MANAGER,
) -> Option<TCPPackage> {
    let setup_no = match serde_json::from_slice(payload) {
        Ok(no) => no,
        Err(_) => return None,
    };

    let vse_setup = {
        let _dkg_manager = dkg_manager.lock().await;
        match _dkg_manager.directory(setup_no) {
            Some(dir) => {
                let _dir = dir.lock().await;
                _dir.setup().clone()
            }
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

async fn handle_request_dkg_packages(
    timestamp: i64,
    payload: &[u8],
    dkg_manager: &DKG_MANAGER,
    keys: &KeyHolder,
) -> Option<TCPPackage> {
    if payload.len() != 16 {
        return None;
    }

    let (setup_no, package_count): (u64, u64) = match serde_json::from_slice(payload) {
        Ok(tuple) => tuple,
        Err(_) => return None,
    };

    let vse_setup = {
        let _dkg_manager = dkg_manager.lock().await;
        match _dkg_manager.directory(setup_no) {
            Some(dir) => {
                let _dir = dir.lock().await;
                _dir.setup().clone()
            }
            None => return None,
        }
    };

    let mut auth_packages = Vec::<Authenticable<DKGPackage>>::with_capacity(package_count as usize);

    for _ in 0..package_count {
        let package = match DKGPackage::new(keys.secret_key(), &vse_setup.signatories()) {
            Some(package) => package,
            None => return None,
        };
        let auth_package = match Authenticable::new(package, keys.secret_key()) {
            Some(auth_package) => auth_package,
            None => return None,
        };

        auth_packages.push(auth_package);
    }

    let response_payload = match serde_json::to_vec(&auth_packages) {
        Ok(tuple) => tuple,
        Err(_) => return None,
    };

    let response_package = {
        let kind = PackageKind::RequestDKGPackages;
        TCPPackage::new(kind, timestamp, &response_payload)
    };

    Some(response_package)
}

async fn handle_deliver_dkg_sessions(
    timestamp: i64,
    payload: &[u8],
    dkg_manager: &mut DKG_MANAGER,
) -> Option<TCPPackage> {
    if payload.len() <= 8 {
        return None;
    }

    let (dir_height, dkg_sessions): (u64, Vec<DKGSession>) = match serde_json::from_slice(payload) {
        Ok(tuple) => tuple,
        Err(_) => return None,
    };

    let dkg_dir = {
        let _dkg_manager = dkg_manager.lock().await;
        _dkg_manager.directory(dir_height)
    }?;

    let mut response_code = [0x01u8];

    for dkg_session in dkg_sessions.iter() {
        let mut _dkg_dir = dkg_dir.lock().await;
        if !_dkg_dir.insert_session_filled(dkg_session) {
            response_code = [0x00u8]; // Failure code;
            break;
        }
    }

    let response_package = {
        let kind = PackageKind::DeliverDKGSessions;
        let payload = response_code;

        TCPPackage::new(kind, timestamp, &payload)
    };

    Some(response_package)
}

async fn handle_request_partial_sigs(
    timestamp: i64,
    payload: &[u8],
    dkg_manager: &mut DKG_MANAGER,
    keys: &KeyHolder,
) -> Option<TCPPackage> {
    if payload.len() <= 8 {
        return None;
    }

    let (dir_height, requests): (u64, Vec<(u64, [u8; 32])>) = match serde_json::from_slice(payload)
    {
        Ok(tuple) => tuple,
        Err(_) => return None,
    };

    let dkg_directory: DKG_DIRECTORY = {
        let _dkg_manager = dkg_manager.lock().await;
        match _dkg_manager.directory(dir_height) {
            Some(directory) => directory,
            None => return None,
        }
    };

    let mut partial_sigs = Vec::<Scalar>::with_capacity(requests.len());

    for (nonce_index, message) in requests {
        let signing_session = {
            let mut _dkg_directory = dkg_directory.lock().await;
            match _dkg_directory.signing_session(message, nonce_index) {
                Some(directory) => directory,
                None => return None,
            }
        };

        match signing_session.partial_sign(keys.secret_key()) {
            Some(partial_sig) => partial_sigs.push(partial_sig),
            None => return None,
        };
    }

    let response_payload = match serde_json::to_vec(&partial_sigs) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let response_package = {
        let kind = PackageKind::RequestPartialSigs;
        TCPPackage::new(kind, timestamp, &response_payload)
    };

    Some(response_package)
}
