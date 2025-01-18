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
            let mut tasks = vec![];

            for peer in operator_peers {
                let peer = Arc::clone(&peer);
                let dir_height = dir_height.clone();
                let signing_requests = signing_requests.clone();
                let operator_key = peer.key().await;
                let signing_sessions_ = Arc::clone(&signing_sessions_);

                tasks.push(tokio::spawn(async move {
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
                }));
            }

            join_all(tasks).await;
        }

        // #9 Return signing sessions.
        let signing_sessions = {
            let _signing_sessions_ = signing_sessions_.lock().await;
            (*_signing_sessions_).clone()
        };

        // #10 Fill full signatures.
        for signing_session in signing_sessions {
            let full_sig = match signing_session.full_aggregated_sig_bytes() {
                Some(sig) => sig,
                None => return Err(DKGSignError::AggSigErr),
            };
            full_signatures.push(full_sig);
        }

        // # 11 Return full signatures.
        Ok(full_signatures)
    }
}

pub async fn preprocess(peer_manager: &mut PEER_MANAGER, dkg_directory: &DKG_DIRECTORY) {
    println!("run_preprocessing");
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
        Err(_) => panic!("signatory keys into_xpoint_vec"),
    };

    let operator_peers: Vec<PEER> = loop {
        // #4 Connect to operator peers.
        peer_manager
            .add_peers(crate::peer::PeerKind::Operator, &operator_keys)
            .await;

        // #5 Return operator peers.
        let peers: Vec<PEER> = match {
            let _peer_manager = peer_manager.lock().await;
            _peer_manager.retrieve_peers(&operator_keys)
        } {
            Some(peers) => peers,
            None => return,
        };

        match peers.len() >= (operator_keys.len() / 2 + 1) {
            true => break peers,
            false => {
                tokio::time::sleep(Duration::from_millis(1_000)).await;
                continue;
            }
        }
    };

    'preprocess_iter: loop {
        println!("preprocess iter h: {}", dir_height);

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

        println!("is_key_session: {}", is_key_session);

        // #4 Determine number of DKG sessions to be filled.
        let fill_count = match is_key_session {
            true => 1,
            false => NONCE_POOL_FILL,
        };

        println!("fill_count: {}", fill_count);

        // #5 Initialize DKG sessions to fill.
        let dkg_sessions = {
            let mut dkg_sessions = Vec::<DKG_SESSION>::with_capacity(fill_count as usize);
            for _ in 0..fill_count {
                let dkg_session = {
                    let mut _dkg_directory = dkg_directory.lock().await;
                    match _dkg_directory.new_session_to_fill() {
                        Some(session) => Arc::new(Mutex::new(session)),
                        None => return,
                    }
                };

                dkg_sessions.push(dkg_session);
            }

            Arc::new(Mutex::new(dkg_sessions))
        };
        println!("ara 5");

        // #6 Fill DKG sessions with retrieved DKG packages.
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

        // #7 Return DKG sessions.
        let dkg_sessions: Vec<DKG_SESSION> = {
            let _dkg_sessions = dkg_sessions.lock().await;
            (*_dkg_sessions).clone()
        };

        // #8 Initialize final DKG sessions list.
        let mut final_dkg_sessions = Vec::<DKGSession>::new();

        println!("ara 8");
        // #9 Insert DKG sessions to the directory.
        {
            let mut _dkg_directory = dkg_directory.lock().await;
            for session in dkg_sessions.iter() {
                let session = {
                    let _session = session.lock().await;
                    (*_session).clone()
                };

                if _dkg_directory.insert_session_filled(&session) {
                    final_dkg_sessions.push(session);
                }
            }
        }

        println!("final_dkg_sessions len is: {}", final_dkg_sessions.len());

        // #10 Check valid DKG sessions length.
        if final_dkg_sessions.len() == 0 {
            continue 'preprocess_iter;
        }

        println!("ara 9");
        // #11 Deliver DKG sessions.
        {
            let mut tasks = vec![];

            for peer in operator_peers.iter() {
                let peer = Arc::clone(&peer);
                let dir_height = dir_height.clone();
                let final_dkg_sessions = final_dkg_sessions.clone();

                tasks.push(tokio::spawn(async move {
                    let _ = peer
                        .deliver_dkg_sessions(dir_height, final_dkg_sessions)
                        .await;
                }));
            }

            join_all(tasks).await;
        }
        println!("done");
        tokio::time::sleep(Duration::from_millis(1_000)).await;
    }
}
