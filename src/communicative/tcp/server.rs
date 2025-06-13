use super::package::{PackageKind, TCPPackage};
use super::tcp::{self, port_number};
use crate::communicative::nns::client::NNSClient;
use crate::communicative::peer::manager::coordinator_key;
use crate::communicative::peer::peer::SOCKET;
use crate::operative::session::ccontext::{CSessionStage, CSESSION_CTX};
use crate::operative::session::commit::NSessionCommit;
use crate::operative::session::opcov::CSessionOpCov;
use crate::operative::session::uphold::NSessionUphold;
use crate::operative::session::upholdack::CSessionUpholdAck;
use crate::operative::session::upholdnack::CSessionUpholdNack;
use crate::operative::{Chain, OperatingMode};
use crate::transmutative::key::{KeyHolder, ToNostrKeyStr};
use crate::transmutative::musig::session::MusigSessionCtx;
use crate::transmutative::noist::dkg::directory::DKG_DIRECTORY;
use crate::transmutative::noist::dkg::package::DKGPackage;
use crate::transmutative::noist::dkg::session::DKGSession;
use crate::transmutative::noist::manager::DKG_MANAGER;
use crate::transmutative::noist::setup::{keymap::VSEKeyMap, setup::VSESetup};
use crate::transmutative::secp::authenticable::Authenticable;
use crate::transmutative::secp::into::IntoPointVec;
use colored::Colorize;
use secp::Scalar;
use std::{sync::Arc, time::Duration};
use tokio::time::{sleep, Instant};
use tokio::{net::TcpListener, sync::Mutex};

/// Idle client timeout.
#[allow(non_camel_case_types)]
pub const IDLE_CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

/// Payload read timeout.
#[allow(non_camel_case_types)]
pub const PAYLOAD_READ_TIMEOUT: Duration = Duration::from_millis(3000);

/// Payload write timeout.
#[allow(non_camel_case_types)]
pub const PAYLOAD_WRITE_TIMEOUT: Duration = Duration::from_millis(10_000);

pub async fn run(
    mode: OperatingMode,
    chain: Chain,
    nns_client: &NNSClient,
    keys: &KeyHolder,
    dkg_manager: &DKG_MANAGER,
    csession_ctx: Option<CSESSION_CTX>,
) {
    let port_number = port_number(chain);
    let addr = format!("{}:{}", "0.0.0.0", port_number);
    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(_) => {
            eprintln!("{}", format!("Failed to bind {}.", addr).red());

            return;
        }
    };

    match mode {
        OperatingMode::Coordinator => {
            if let None = csession_ctx {
                return; // This is only a coordinator job.
            }

            loop {
                let (socket_, _) = match listener.accept().await {
                    Ok(conn) => (conn.0, conn.1),
                    Err(_) => continue,
                };

                let socket = Arc::new(Mutex::new(socket_));

                tokio::spawn({
                    let socket = Arc::clone(&socket);
                    let keys = keys.clone();
                    let mut dkg_manager = Arc::clone(&dkg_manager);
                    let session_ctx = csession_ctx.clone();

                    async move {
                        handle_socket(&socket, None, mode, &keys, &mut dkg_manager, &session_ctx)
                            .await;
                    }
                });
            }
        }
        OperatingMode::Operator => {
            if let Some(_) = csession_ctx {
                return; // This is not an operator job.
            }

            let coordinator_key = coordinator_key(chain);
            let coordinator_npub = match coordinator_key.to_npub() {
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
                            let csession_ctx = csession_ctx.clone();

                            async move {
                                handle_socket(
                                    &socket,
                                    Some(&socket_alive),
                                    mode,
                                    &keys,
                                    &mut dkg_manager,
                                    &csession_ctx,
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
    csession_ctx: &Option<CSESSION_CTX>,
) {
    loop {
        let package = {
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

            package
        };

        // Process the request kind.
        handle_package(package, socket, mode, keys, dkg_manager, csession_ctx).await;
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
    socket: &SOCKET,
    mode: OperatingMode,
    keys: &KeyHolder,
    dkg_manager: &mut DKG_MANAGER,
    csession_ctx: &Option<CSESSION_CTX>,
) {
    let response_package_ = {
        match mode {
            OperatingMode::Coordinator => match package.kind() {
                PackageKind::Ping => handle_ping(package.timestamp(), &package.payload()).await,
                PackageKind::SyncDKGDir => {
                    handle_sync_dkg_dir(package.timestamp(), &package.payload(), dkg_manager).await
                }

                PackageKind::CommitSession => {
                    handle_commit_session(package.timestamp(), &package.payload(), csession_ctx)
                        .await
                }

                PackageKind::UpholdSession => {
                    handle_uphold_session(package.timestamp(), &package.payload(), csession_ctx)
                        .await
                }

                _ => return,
            },
            OperatingMode::Operator => match package.kind() {
                PackageKind::Ping => handle_ping(package.timestamp(), &package.payload()).await,
                PackageKind::RequestVSEKeymap => {
                    handle_request_vse_keymap(
                        package.timestamp(),
                        &package.payload(),
                        keys,
                        dkg_manager,
                    )
                    .await
                }

                PackageKind::DeliverVSESetup => {
                    handle_deliver_vse_setup(package.timestamp(), &package.payload(), dkg_manager)
                        .await
                }

                PackageKind::SyncDKGDir => {
                    handle_sync_dkg_dir(package.timestamp(), &package.payload(), dkg_manager).await
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
                }
                _ => return,
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
    _dkg_manager: &mut DKG_MANAGER,
) -> Option<TCPPackage> {
    let signatory_keys: Vec<[u8; 32]> = match serde_json::from_slice(payload) {
        Ok(no) => no,
        Err(_) => return None,
    };

    // TODO BRING BACK.
    // Check if requested signatory keys are within the subset of liquidity providers.
    //{
    //let lp_dir: LP_DIRECTORY = {
    //    let _dkg_manager = dkg_manager.lock().await;
    //    _dkg_manager.lp_directory()
    //};

    //let _lp_dir = lp_dir.lock().await;
    //if !_lp_dir.is_valid_subset(&signatory_keys) {
    //    return None;
    //}
    //}

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

async fn handle_sync_dkg_dir(
    timestamp: i64,
    payload: &[u8],
    dkg_manager: &DKG_MANAGER,
) -> Option<TCPPackage> {
    let dir_height: u64 = match serde_json::from_slice(payload) {
        Ok(no) => no,
        Err(_) => return None,
    };

    let (setup, sessions) = {
        let _dkg_manager = dkg_manager.lock().await;
        match _dkg_manager.directory_by_height(dir_height) {
            Some(dir) => {
                let _dir = dir.lock().await;
                (_dir.setup(), _dir.sessions())
            }
            None => return None,
        }
    };

    let payload = match serde_json::to_vec(&(setup, sessions)) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let response_package = {
        let kind = PackageKind::SyncDKGDir;
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
    let (setup_no, package_count): (u64, u64) = match serde_json::from_slice(payload) {
        Ok(tuple) => tuple,
        Err(_) => return None,
    };

    let vse_setup = {
        let _dkg_manager = dkg_manager.lock().await;
        match _dkg_manager.directory_by_height(setup_no) {
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
        let auth_package = match Authenticable::new(package, keys.secret_key().serialize()) {
            Some(auth_package) => auth_package,
            None => return None,
        };

        auth_packages.push(auth_package);
    }

    let response_payload = match serde_json::to_vec(&auth_packages) {
        Ok(bytes) => bytes,
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
    let (dir_height, dkg_sessions): (u64, Vec<DKGSession>) = match serde_json::from_slice(payload) {
        Ok(tuple) => tuple,
        Err(_) => return None,
    };

    let dkg_dir = {
        let _dkg_manager = dkg_manager.lock().await;
        _dkg_manager.directory_by_height(dir_height)
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

/// Coordinator asking operators for partial signatures generic.
async fn handle_request_partial_sigs(
    timestamp: i64,
    payload: &[u8],
    dkg_manager: &mut DKG_MANAGER,
    keys: &KeyHolder,
) -> Option<TCPPackage> {
    let (dir_height, requests): (u64, Vec<(u64, [u8; 32], Option<MusigSessionCtx>)>) =
        match serde_json::from_slice(payload) {
            Ok(triple) => triple,
            Err(_) => return None,
        };

    let dkg_directory: DKG_DIRECTORY = {
        let _dkg_manager = dkg_manager.lock().await;
        match _dkg_manager.directory_by_height(dir_height) {
            Some(directory) => directory,
            None => return None,
        }
    };

    let mut partial_sigs = Vec::<Scalar>::with_capacity(requests.len());

    for (nonce_index, message, musig_nesting_ctx) in requests {
        let signing_session = {
            let mut _dkg_directory = dkg_directory.lock().await;
            match _dkg_directory.signing_session(message, nonce_index, musig_nesting_ctx, true) {
                Some(directory) => directory,
                None => return None,
            }
        };

        match signing_session.partial_sign(keys.secret_key().serialize()) {
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

/// Coordinator asking operators for partial signatures
async fn _handle_request_opcov(
    timestamp: i64,
    payload: &[u8],
    dkg_manager: &mut DKG_MANAGER,
    keys: &KeyHolder,
) -> Option<TCPPackage> {
    let opcov: CSessionOpCov = match serde_json::from_slice(payload) {
        Ok(opcov) => opcov,
        Err(_) => return None,
    };

    let opcovack = opcov.opcovack(dkg_manager, keys).await?;

    let response_payload = match serde_json::to_vec(&opcovack) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let response_package = {
        let kind = PackageKind::RequestOpCov;
        TCPPackage::new(kind, timestamp, &response_payload)
    };

    Some(response_package)
}

/// Coordinator handling msg.sender's session commitment request.
async fn handle_commit_session(
    timestamp: i64,
    payload: &[u8],
    csession_ctx: &Option<CSESSION_CTX>,
) -> Option<TCPPackage> {
    let csession_ctx: CSESSION_CTX = Arc::clone(&csession_ctx.to_owned()?);
    let auth_commit: Authenticable<NSessionCommit> = serde_json::from_slice(payload).ok()?;
    let account = auth_commit.object().account();

    // Wait until the session is on.
    loop {
        let stage = { csession_ctx.lock().await.stage() };

        match stage {
            CSessionStage::On => break,
            _ => sleep(Duration::from_millis(10)).await,
        }
    }

    let commit_result = {
        // Insert commit.
        let insert_commit_result = {
            let mut _csession_ctx = csession_ctx.lock().await;
            _csession_ctx.insert_commit(&auth_commit).await
        };

        // Check the insertion result.
        match insert_commit_result {
            Ok(_) => {
                // Inserted to commit pool, wait until the session is locked.
                loop {
                    let stage = { csession_ctx.lock().await.stage() };

                    match stage {
                        CSessionStage::Locked => break,
                        _ => sleep(Duration::from_millis(10)).await,
                    }
                }

                // Return the commitack upon the session is locked.
                {
                    let mut _csession_ctx = csession_ctx.lock().await;
                    _csession_ctx.commitack(account)
                }
            }
            // Return the nack if the commit is invalid.
            Err(commit_nack) => Err(commit_nack),
        }
    };

    let response_payload = match serde_json::to_vec(&commit_result) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let response_package = {
        let kind = PackageKind::CommitSession;
        TCPPackage::new(kind, timestamp, &response_payload)
    };

    Some(response_package)
}

/// Coordinator handling msg.sender's session uphold.
async fn handle_uphold_session(
    timestamp: i64,
    payload: &[u8],
    csession_ctx: &Option<CSESSION_CTX>,
) -> Option<TCPPackage> {
    let csession_ctx: CSESSION_CTX = Arc::clone(&csession_ctx.to_owned()?);
    let auth_uphold: Authenticable<NSessionUphold> = serde_json::from_slice(payload).ok()?;
    let msg_sender = auth_uphold.object().msg_sender();

    // Check if the session is locked.
    {
        let stage = { csession_ctx.lock().await.stage() };

        if stage != CSessionStage::Locked {
            return None;
        }
    }

    let uphold_result: Result<CSessionUpholdAck, CSessionUpholdNack> = {
        let insert_uphold_result = {
            let mut _csession_ctx = csession_ctx.lock().await;
            _csession_ctx.insert_uphold(auth_uphold)
        };

        match insert_uphold_result {
            Ok(_) => {
                // Uphold inserted, now wait until stage status changes.
                loop {
                    let stage = { csession_ctx.lock().await.stage() };

                    match stage {
                        CSessionStage::Locked => sleep(Duration::from_millis(10)).await,
                        _ => {
                            // Stage status has changed; either to upheld or back to on (due to failure).

                            let upholdack_result = {
                                let mut _csession_ctx = csession_ctx.lock().await;
                                _csession_ctx.upholdack(msg_sender)
                            };

                            match upholdack_result {
                                Ok(upholdack) => {
                                    break Ok(upholdack);
                                }

                                Err(uphold_nack) => {
                                    break Err(uphold_nack);
                                }
                            };
                        }
                    }
                }
            }
            Err(uphold_nack) => Err(uphold_nack),
        }
    };

    let response_payload = match serde_json::to_vec(&uphold_result) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let response_package = {
        let kind = PackageKind::UpholdSession;
        TCPPackage::new(kind, timestamp, &response_payload)
    };

    Some(response_package)
}
