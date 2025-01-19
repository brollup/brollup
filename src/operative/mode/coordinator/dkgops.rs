use crate::{
    into::{IntoPoint, IntoPointByteVec, IntoPointVec},
    liquidity,
    noist::{
        dkg::{directory::SigningSession, session::DKGSession},
        setup::setup::VSESetup,
    },
    peer::PeerConnection,
    peer_manager::PeerManagerExt,
    tcp::client::TCPClient,
    DKG_DIRECTORY, DKG_MANAGER, DKG_SESSION, PEER, PEER_MANAGER,
};
use async_trait::async_trait;
use colored::Colorize;
use futures::future::join_all;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

const NONCE_POOL_THRESHOLD: u64 = 512;
const NONCE_POOL_FILL: u64 = 64;

#[derive(Clone, Debug)]
pub enum DKGSetupError {
    PeerRetrievalErr,
    InsufficientPeers,
    InsufficientKeymaps,
    PreSetupInitErr,
    PostSetupVerifyErr,
    ManagerInsertionErr,
}

#[derive(Clone, Debug)]
pub enum DKGSignError {
    DirectoryNotFound,
    RetrievePeersErr,
    BelowThresholdPeers,
    PickSigningSessionErr,
    AggSigErr,
    PartialSignTimeout,
}

#[async_trait]
pub trait DKGOps {
    async fn new_setup(&self, peer_manager: &mut PEER_MANAGER) -> Result<u64, DKGSetupError>;
    async fn run_preprocessing(&self, peer_manager: &mut PEER_MANAGER);
    async fn sign(
        &self,
        peer_manager: &mut PEER_MANAGER,
        dir_height: u64,
        messages: Vec<[u8; 32]>,
    ) -> Result<Vec<[u8; 64]>, DKGSignError>;
}

#[async_trait]
impl DKGOps for DKG_MANAGER {
    async fn new_setup(&self, peer_manager: &mut PEER_MANAGER) -> Result<u64, DKGSetupError> {
        // #1 Pick a setup number.
        let dir_height = {
            let _dkg_manager = self.lock().await;
            _dkg_manager.setup_height() + 1
        };

        // #2 Retrieve the liquidity provider list.
        let lp_keys = liquidity::provider::provider_list();

        // #3 Connect to liquidity providers (if possible).
        let lp_peers: Vec<PEER> = match {
            peer_manager
                .add_peers(crate::peer::PeerKind::Operator, &lp_keys)
                .await;

            let mut _peer_manager = peer_manager.lock().await;
            _peer_manager.retrieve_peers(&lp_keys)
        } {
            Some(some) => some,
            None => return Err(DKGSetupError::PeerRetrievalErr),
        };

        // #4 Check if there are enough peer connections.
        if lp_peers.len() <= lp_keys.len() / 20 {
            return Err(DKGSetupError::InsufficientPeers);
        }

        // #5 Convert LP keys into secp Points.
        let lp_key_points = match lp_keys.into_point_vec() {
            Ok(points) => points,
            Err(_) => return Err(DKGSetupError::PreSetupInitErr),
        };

        // #6 Initialize VSE setup with the list of LP keys.
        let vse_setup_ = match VSESetup::new(&lp_key_points, dir_height) {
            Some(setup) => Arc::new(Mutex::new(setup)),
            None => return Err(DKGSetupError::PreSetupInitErr),
        };

        // #7 Retrieve VSE Keymap's from each connected LP peer.
        {
            let mut tasks = vec![];

            for lp_peer in lp_peers.clone() {
                let vse_setup_ = Arc::clone(&vse_setup_);
                let lp_keys = lp_keys.clone();

                let lp_key = lp_peer.key().await;

                tasks.push(tokio::spawn(async move {
                    let keymap = match lp_peer.request_vse_keymap(&lp_keys).await {
                        Ok(keymap) => keymap,
                        Err(_) => return,
                    };

                    if keymap.signatory().serialize_xonly() == lp_key {
                        let mut _vse_setup_ = vse_setup_.lock().await;
                        _vse_setup_.insert_keymap(keymap);
                    }
                }));
            }

            join_all(tasks).await;
        }

        // #8 Return the original VSE Setup struct.
        let mut vse_setup = {
            let _vse_setup = vse_setup_.lock().await;
            (*_vse_setup).clone()
        };

        // #9 Check if there are enough number of keymaps.
        if vse_setup.map_len() <= lp_peers.len() / 2 {
            return Err(DKGSetupError::InsufficientKeymaps);
        }

        // #10 Remove liquidity providers that failed to connect.
        vse_setup.remove_missing();

        // #11 Verify the final VSE setup.
        if !vse_setup.verify() {
            return Err(DKGSetupError::PostSetupVerifyErr);
        };

        // #12 Deliver VSE setup to each connected liquidity provider.
        {
            let mut tasks = vec![];

            for lp_peer in lp_peers.clone() {
                let lp_key = lp_peer.key().await;

                if vse_setup.is_signatory(lp_key) {
                    let vse_setup = vse_setup.clone();

                    tasks.push(tokio::spawn(async move {
                        let _ = lp_peer.deliver_vse_setup(&vse_setup).await;
                    }));
                }
            }

            join_all(tasks).await;
        }

        // #13 Insert VSE setup to local DKG directory and return the directory.
        let dkg_directory = match {
            let mut _dkg_manager = self.lock().await;

            if !_dkg_manager.insert_setup(&vse_setup) {
                return Err(DKGSetupError::ManagerInsertionErr);
            }

            _dkg_manager.directory(vse_setup.height())
        } {
            Some(directory) => directory,
            None => return Err(DKGSetupError::ManagerInsertionErr),
        };

        // #14 Run preprovessing for the new directory.
        {
            let mut peer_manager = Arc::clone(&peer_manager);
            let dkg_directory = Arc::clone(&dkg_directory);
            tokio::spawn(async move {
                let _ = preprocess(&mut peer_manager, &dkg_directory).await;
            });
        }

        // #15 Return the directory height.
        Ok(dir_height)
    }

    async fn run_preprocessing(&self, peer_manager: &mut PEER_MANAGER) {
        let dkg_directories = {
            let _dkg_manager = self.lock().await;
            _dkg_manager.directories()
        };

        for (_, dkg_directory) in dkg_directories {
            let mut peer_manager = Arc::clone(&peer_manager);
            let dkg_directory = Arc::clone(&dkg_directory);
            tokio::spawn(async move {
                preprocess(&mut peer_manager, &dkg_directory).await;
            });
        }
    }

    async fn sign(
        &self,
        peer_manager: &mut PEER_MANAGER,
        dir_height: u64,
        messages: Vec<[u8; 32]>,
    ) -> Result<Vec<[u8; 64]>, DKGSignError> {
        // #1 Initialize full signatures list.
        let mut full_signatures = Vec::<[u8; 64]>::new();

        // # 2 Initialize DKG directory.
        let dkg_directory: DKG_DIRECTORY = {
            let _dkg_manager = self.lock().await;
            match _dkg_manager.directory(dir_height) {
                Some(directory) => directory,
                None => return Err(DKGSignError::DirectoryNotFound),
            }
        };

        // #3 Return operator keys.
        let operator_keys = {
            let _dkg_directory = dkg_directory.lock().await;
            match _dkg_directory.setup().signatories().into_xpoint_vec() {
                Ok(vec) => vec,
                Err(_) => panic!("signatory keys into_xpoint_vec"),
            }
        };

        // #4 Return operator peers.
        let operator_peers: Vec<PEER> = {
            let peers: Vec<PEER> = match {
                let _peer_manager = peer_manager.lock().await;
                _peer_manager.retrieve_peers(&operator_keys)
            } {
                Some(peers) => peers,
                None => return Err(DKGSignError::RetrievePeersErr),
            };

            match peers.len() >= (operator_keys.len() / 2 + 1) {
                true => peers,
                false => return Err(DKGSignError::BelowThresholdPeers),
            }
        };

        // #5 Initialize signing sessions.
        let mut signing_sessions = Vec::<SigningSession>::with_capacity(messages.len());
        let mut signing_requests = Vec::<(u64, [u8; 32])>::with_capacity(signing_sessions.len()); // Nonce index, message.

        // #6 Pick fresh signing sessions to be filled.
        for message in messages.iter() {
            let mut _dkg_directory = dkg_directory.lock().await;
            let signing_session = match _dkg_directory.pick_signing_session(message.to_owned()) {
                Some(session) => session,
                None => return Err(DKGSignError::PickSigningSessionErr),
            };

            signing_requests.push((signing_session.nonce_index(), message.to_owned()));
            signing_sessions.push(signing_session);
        }

        // #7 Guard the signing sessions list to collect partial signatures.
        let signing_sessions_ = Arc::new(Mutex::new(signing_sessions));

        // #8 Request partial signatures and fill signing sessions with them.
        {
            for peer in operator_peers {
                let peer = Arc::clone(&peer);
                let dir_height = dir_height.clone();
                let signing_requests = signing_requests.clone();
                let operator_key = peer.key().await;
                let signing_sessions_ = Arc::clone(&signing_sessions_);

                tokio::spawn(async move {
                    let partial_sigs = match peer
                        .request_partial_sigs(dir_height, &signing_requests)
                        .await
                    {
                        Ok(partial_sigs) => partial_sigs,
                        Err(_) => return,
                    };

                    if partial_sigs.len() != signing_requests.len() {
                        return;
                    }

                    let operator = match operator_key.into_point() {
                        Ok(point) => point,
                        Err(_) => return,
                    };

                    for (index, partial_sig) in partial_sigs.iter().enumerate() {
                        let mut _signing_sessions_ = signing_sessions_.lock().await;
                        if !_signing_sessions_[index]
                            .insert_partial_sig(operator, partial_sig.to_owned())
                        {
                            return;
                        }
                    }
                });
            }
        }

        // #9 Check if the threshold is met among all sessions.
        tokio::select! {
            // Timeout is 500 ms base plus 15 ms for each requested signature.
            _ = tokio::time::sleep( Duration::from_millis(500 + (signing_requests.len() as u64) * 15)) => {
                return Err(DKGSignError::PartialSignTimeout);
            },

            _ = async {
                loop {
                    let all_above_threshold = {
                        let signing_sessions = signing_sessions_.lock().await;
                        signing_sessions
                            .iter()
                            .all(|session| session.is_threshold_met())
                    };

                    if all_above_threshold {
                        // Exit the loop if the threshold is met
                        break;
                    }

                    // Small sleep to avoid busy-looping
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            } => {}
        }

        // #10 Return signing sessions.
        let signing_sessions = {
            let _signing_sessions_ = signing_sessions_.lock().await;
            (*_signing_sessions_).clone()
        };

        // #11 Fill full signatures.
        for signing_session in signing_sessions {
            let full_sig = match signing_session.full_aggregated_sig_bytes() {
                Some(sig) => sig,
                None => return Err(DKGSignError::AggSigErr),
            };
            full_signatures.push(full_sig);
        }

        // # 12 Return full signatures.
        Ok(full_signatures)
    }
}

pub async fn preprocess(peer_manager: &mut PEER_MANAGER, dkg_directory: &DKG_DIRECTORY) {
    // #1 Return VSE setup.
    let setup = {
        let _dkg_directory = dkg_directory.lock().await;
        _dkg_directory.setup().clone()
    };

    // #2 Return directory height.
    let dir_height = setup.height();

    // #3 Return operator keys.
    let operator_keys = match setup.signatories().into_xpoint_vec() {
        Ok(vec) => vec,
        Err(_) => panic!("Unexpected into_xpoint_vec err."),
    };

    // #4 Connect to operator peers and return the list.
    let operator_peers: Vec<PEER> = loop {
        peer_manager
            .add_peers(crate::peer::PeerKind::Operator, &operator_keys)
            .await;

        let peers: Vec<PEER> = match {
            let _peer_manager = peer_manager.lock().await;
            _peer_manager.retrieve_peers(&operator_keys)
        } {
            Some(peers) => peers,
            None => {
                eprintln!(
                    "{}",
                    format!(
                        "DIR HEIGHT '{}' PREPROCESS LOG: Failed to retrieve peers. Re-trying in 5..",
                        dir_height
                    )
                    .yellow()
                );
                tokio::time::sleep(Duration::from_millis(5_000)).await;
                continue;
            }
        };

        match peers.len() >= (operator_keys.len() / 2 + 1) {
            true => break peers,
            false => {
                eprintln!(
                    "{}",
                    format!(
                        "DIR HEIGHT '{}' PREPROCESS LOG: Below threshold peers. Re-trying in 5..",
                        dir_height
                    )
                    .yellow()
                );
                tokio::time::sleep(Duration::from_millis(5_000)).await;
                continue;
            }
        }
    };

    'preprocess_iter: loop {
        // #1 Return the number of available DKG sessions.
        let num_available_sessions = {
            let _dkg_directory = dkg_directory.lock().await;
            _dkg_directory.available_sessions()
        };

        // #2 If enough DKG sessions available skip preprocessing.
        if num_available_sessions > NONCE_POOL_THRESHOLD {
            tokio::time::sleep(Duration::from_millis(500)).await;
            continue 'preprocess_iter;
        }

        // #3 Determine if this is a key session.
        let is_key_session = {
            let _dkg_directory = dkg_directory.lock().await;
            match _dkg_directory.group_key_session() {
                Some(_) => false,
                None => true,
            }
        };

        // #4 Determine number of DKG sessions to be filled.
        let fill_count = match is_key_session {
            true => 1,
            false => NONCE_POOL_FILL,
        };

        // #5 Return DKG sessions filled.
        let dkg_sessions: Vec<DKG_SESSION> = loop {
            let dkg_sessions = {
                let mut dkg_sessions = Vec::<DKG_SESSION>::with_capacity(fill_count as usize);

                for _ in 0..fill_count {
                    let dkg_session = {
                        let mut _dkg_directory = dkg_directory.lock().await;

                        match _dkg_directory.new_session_to_fill() {
                            Some(session) => Arc::new(Mutex::new(session)),
                            None => panic!("Unexpected new_session_to_fill err."),
                        }
                    };

                    dkg_sessions.push(dkg_session);
                }

                Arc::new(Mutex::new(dkg_sessions))
            };

            {
                let mut tasks = vec![];

                for peer in operator_peers.iter() {
                    let peer = Arc::clone(&peer);
                    let setup = setup.clone();
                    let dkg_sessions = Arc::clone(&dkg_sessions);

                    tasks.push(tokio::spawn(async move {
                        let auth_packages =
                            match peer.request_dkg_packages(dir_height, fill_count).await {
                                Ok(packages) => packages,
                                Err(_) => return,
                            };

                        if auth_packages.len() != fill_count as usize {
                            return;
                        }

                        for (index, auth_package) in auth_packages.iter().enumerate() {
                            let dkg_session = {
                                let _dkg_sessions = dkg_sessions.lock().await;
                                Arc::clone(&_dkg_sessions[index])
                            };

                            {
                                let mut _dkg_session = dkg_session.lock().await;
                                _dkg_session.insert(&auth_package, &setup);
                            }
                        }
                    }));
                }

                join_all(tasks).await;
            }

            let dkg_sessions: Vec<DKG_SESSION> = {
                let _dkg_sessions = dkg_sessions.lock().await;
                (*_dkg_sessions).clone()
            };

            for dkg_session in dkg_sessions.iter() {
                let _dkg_session = dkg_session.lock().await;
                if !_dkg_session.is_threshold_met() {
                    eprintln!(
                        "{}",
                        format!(
                            "DIR HEIGHT '{}' PREPROCESS LOG: DKG Session '{}' threshold not met. Re-trying in 5..",
                            dir_height, _dkg_session.index()
                        )
                        .yellow()
                    );
                    tokio::time::sleep(Duration::from_millis(5_000)).await;
                    continue;
                }
            }

            break dkg_sessions;
        };

        // #6 Initialize inserted DKG sessions list.
        let mut inserted_dkg_sessions = Vec::<DKGSession>::new();

        // #7 Insert DKG sessions to the directory.
        {
            let mut _dkg_directory = dkg_directory.lock().await;
            for session in dkg_sessions.iter() {
                let session = {
                    let _session = session.lock().await;
                    (*_session).clone()
                };

                match _dkg_directory.insert_session_filled(&session) {
                    true => inserted_dkg_sessions.push(session),
                    false => {
                        eprintln!(
                            "{}",
                            format!(
                                "DIR HEIGHT '{}' PREPROCESS LOG: DKG Session '{}' insertion error. Re-iterating.",
                                dir_height, session.index()
                            )
                            .yellow()
                        );
                        continue 'preprocess_iter;
                    }
                }
            }
        }

        let num_deliveries = Arc::new(Mutex::new(0u64));

        // #8 Deliver DKG sessions.
        loop {
            {
                let mut tasks = vec![];

                for peer in operator_peers.iter() {
                    let peer = Arc::clone(&peer);
                    let dir_height = dir_height.clone();
                    let final_dkg_sessions = inserted_dkg_sessions.clone();
                    let num_deliveries = Arc::clone(&num_deliveries);

                    tasks.push(tokio::spawn(async move {
                        if let Ok(_) = peer
                            .deliver_dkg_sessions(dir_height, final_dkg_sessions)
                            .await
                        {
                            let mut _num_deliveries = num_deliveries.lock().await;
                            *_num_deliveries += 1;
                        }
                    }));
                }

                join_all(tasks).await;
            }

            let num_deliveries = {
                let _num_deliveries = num_deliveries.lock().await;
                *_num_deliveries
            };

            let min_deliveries = (operator_keys.len() / 2 + 1) as u64;

            match num_deliveries >= min_deliveries {
                true => break,
                false => {
                    eprintln!(
                        "{}",
                        format!(
                            "DIR HEIGHT '{}' PREPROCESS LOG: Delivered '{}'. MIN '{}'. Re-trying in 1..",
                            dir_height, num_deliveries, min_deliveries
                        )
                        .yellow()
                    );
                    tokio::time::sleep(Duration::from_millis(1_000)).await;
                    continue;
                }
            }
        }
    }
}
