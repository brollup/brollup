use crate::{
    into::{IntoPointByteVec, IntoPointVec},
    liquidity,
    noist::{dkg::session::DKGSession, setup::setup::VSESetup},
    peer::PeerConnection,
    peer_manager::PeerManagerExt,
    tcp::client::TCPClient,
    DKG_DIRECTORY, DKG_MANAGER, DKG_SESSION, PEER, PEER_MANAGER,
};
use async_trait::async_trait;
use futures::future::join_all;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

const NONCE_POOL_THRESHOLD: u64 = 256;
const NONCE_POOL_FILL: u64 = 64;

#[derive(Clone, Debug)]
pub enum SignatorySetupError {
    PeerRetrievalErr,
    InsufficientPeers,
    PreSetupInitErr,
    PostSetupVerifyErr,
    ManagerInsertionErr,
}

#[async_trait]
pub trait SignatoryOps {
    async fn coordinate_new_setup(
        &self,
        peer_manager: &mut PEER_MANAGER,
    ) -> Result<u64, SignatorySetupError>;
    async fn coordinate_preprocess(&self, peer_manager: &mut PEER_MANAGER);
}

#[async_trait]
impl SignatoryOps for DKG_MANAGER {
    async fn coordinate_new_setup(
        &self,
        peer_manager: &mut PEER_MANAGER,
    ) -> Result<u64, SignatorySetupError> {
        // #1 Pick a setup number.
        let setup_height = {
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
            None => return Err(SignatorySetupError::PeerRetrievalErr),
        };

        // #4 Check if there are enough peer connections.
        if lp_peers.len() < lp_keys.len() / 10 {
            return Err(SignatorySetupError::InsufficientPeers);
        }

        // #5 Convert LP keys into secp Points.
        let lp_key_points = match lp_keys.into_point_vec() {
            Ok(points) => points,
            Err(_) => return Err(SignatorySetupError::PreSetupInitErr),
        };

        // #6 Initialize VSE setup with the list of LP keys.
        let vse_setup_ = match VSESetup::new(&lp_key_points, setup_height) {
            Some(setup) => Arc::new(Mutex::new(setup)),
            None => return Err(SignatorySetupError::PreSetupInitErr),
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

        // #9 Remove liquidity providers that failed to connect.
        vse_setup.remove_missing();

        // #10 Verify the final VSE setup.
        if !vse_setup.verify() {
            return Err(SignatorySetupError::PostSetupVerifyErr);
        };

        // #11 Deliver VSE setup to each connected liquidity provider.
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

        // #12 Insert VSE setup to local DKG directory and return the directory.
        let dkg_directory = match {
            let mut _dkg_manager = self.lock().await;

            if !_dkg_manager.insert_setup(&vse_setup) {
                return Err(SignatorySetupError::ManagerInsertionErr);
            }

            _dkg_manager.directory(vse_setup.height())
        } {
            Some(directory) => directory,
            None => return Err(SignatorySetupError::ManagerInsertionErr),
        };

        // #13 Run preprovessing for the new directory.
        {
            let mut peer_manager = Arc::clone(&peer_manager);
            let dkg_directory = Arc::clone(&dkg_directory);
            tokio::spawn(async move {
                let _ = run_preprocessing(&mut peer_manager, &dkg_directory).await;
            });
        }

        // #14 Return the VSE setup.
        Ok(setup_height)
    }

    async fn coordinate_preprocess(&self, peer_manager: &mut PEER_MANAGER) {
        let dkg_directories = {
            let _dkg_manager = self.lock().await;
            _dkg_manager.directories()
        };

        for (_, dkg_directory) in dkg_directories {
            let mut peer_manager = Arc::clone(&peer_manager);
            let dkg_directory = Arc::clone(&dkg_directory);
            tokio::spawn(async move {
                run_preprocessing(&mut peer_manager, &dkg_directory).await;
            });
        }
    }
}

pub async fn run_preprocessing(peer_manager: &mut PEER_MANAGER, dkg_directory: &DKG_DIRECTORY) {
    // #1 Return VSE setup.
    let setup = {
        let _dkg_directory = dkg_directory.lock().await;
        _dkg_directory.setup().clone()
    };

    // #2 Return setup height.
    let setup_height = setup.height();

    // #3 Return operator keys.
    let operator_keys = match setup.signatories().into_xpoint_vec() {
        Ok(vec) => vec,
        Err(_) => return,
    };

    // #4 Connect to operator peers.
    peer_manager
        .add_peers(crate::peer::PeerKind::Operator, &operator_keys)
        .await;

    // #5 Return operator peers.
    let operator_peers: Vec<PEER> = match {
        let _peer_manager = peer_manager.lock().await;
        _peer_manager.retrieve_peers(&operator_keys)
    } {
        Some(peers) => peers,
        None => return,
    };

    loop {
        // #1 Return the number of available DKG sessions.
        let num_available_sessions = {
            let _dkg_directory = dkg_directory.lock().await;
            _dkg_directory.available_sessions()
        };

        // #2 If enough DKG sessions available skip preprocessing.
        if num_available_sessions >= NONCE_POOL_THRESHOLD {
            tokio::time::sleep(Duration::from_millis(100)).await;
            continue;
        }

        // #3 Initialize new DKG sessions to fill.
        let mut dkg_sessions_ = Vec::<DKG_SESSION>::with_capacity(NONCE_POOL_FILL as usize);

        for _ in 0..NONCE_POOL_FILL {
            let dkg_session = {
                let mut _dkg_directory = dkg_directory.lock().await;
                match _dkg_directory.new_session_to_fill() {
                    Some(session) => Arc::new(Mutex::new(session)),
                    None => return,
                }
            };

            dkg_sessions_.push(dkg_session);
        }

        let dkg_sessions = Arc::new(Mutex::new(dkg_sessions_));

        // Phase #0: ask operators new DKG packages.

        {
            let mut tasks = vec![];

            for peer in operator_peers.iter() {
                let peer = Arc::clone(&peer);
                let setup = setup.clone();
                let dkg_sessions = Arc::clone(&dkg_sessions);

                tasks.push(tokio::spawn(async move {
                    if let Ok(response) = peer
                        .request_dkg_packages(setup_height, NONCE_POOL_FILL)
                        .await
                    {
                        let dkg_sessions_ = {
                            let _dkg_sessions = dkg_sessions.lock().await;
                            (*_dkg_sessions).clone()
                        };

                        let operator_key = {
                            let _operator = peer.lock().await;
                            _operator.key()
                        };

                        for (index, auth_package) in response.iter().enumerate() {
                            if auth_package.key() == operator_key {
                                if auth_package.authenticate() {
                                    let dkg_session = Arc::clone(&dkg_sessions_[index]);

                                    {
                                        let mut _dkg_session = dkg_session.lock().await;
                                        let _ = _dkg_session.insert(&auth_package, &setup);
                                    }
                                }
                            }
                        }
                    }
                }));
            }

            join_all(tasks).await;
        }

        let mut sessions = Vec::<DKGSession>::new();

        let _dkg_sessions = {
            let _dkg_sessions = dkg_sessions.lock().await;
            (*_dkg_sessions).clone()
        };

        for dkg_session in _dkg_sessions {
            let _dkg_session = dkg_session.lock().await;
            sessions.push((*_dkg_session).clone());
        }

        {
            let mut _dkg_directory = dkg_directory.lock().await;
            for session in sessions {
                let _ = _dkg_directory.insert_session_filled(&session);
            }
        }

        tokio::time::sleep(Duration::from_millis(1_000)).await;
    }
}
