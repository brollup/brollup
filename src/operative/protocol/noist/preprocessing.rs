use crate::{
    noist::dkg::session::DKGSession,
    tcp::{client::TCPClient, peer::PeerListExt},
    DKG_DIRECTORY, DKG_SESSION, NOIST_MANAGER, PEER_LIST,
};
use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Mutex;

const NONCE_POOL_LEN: u64 = 1000;

pub enum PrepeocessingError {
    DirectoryInitErr,
    NewSessionToFillErr,
}

pub async fn run_preprocessing(
    operator_list: &PEER_LIST,
    noist_manager: &NOIST_MANAGER,
    setup_no: u64,
) -> Result<(), PrepeocessingError> {
    let dkg_directory: DKG_DIRECTORY = {
        let _noist_manager = noist_manager.lock().await;
        match _noist_manager.directory(setup_no) {
            Some(dir) => Arc::clone(&dir),
            None => return Err(PrepeocessingError::DirectoryInitErr),
        }
    };

    let vse_setup = {
        let _dkg_directory = dkg_directory.lock().await;
        _dkg_directory.setup().clone()
    };

    loop {
        let num_available_sessions = {
            let _dkg_directory = dkg_directory.lock().await;
            _dkg_directory.available_sessions()
        };

        if num_available_sessions >= NONCE_POOL_LEN {
            continue;
        }

        let num_sessions_to_fill: u64 = 64;

        let mut dkg_sessions_ = Vec::<DKG_SESSION>::with_capacity(num_sessions_to_fill as usize);

        for _ in 0..num_sessions_to_fill {
            let dkg_session = {
                let mut _dkg_directory = dkg_directory.lock().await;
                match _dkg_directory.new_session_to_fill() {
                    Some(session) => Arc::new(Mutex::new(session)),
                    None => return Err(PrepeocessingError::NewSessionToFillErr),
                }
            };

            dkg_sessions_.push(dkg_session);
        }

        let dkg_sessions = Arc::new(Mutex::new(dkg_sessions_));

        // Phase #0: ask operators new DKG packages.

        {
            let mut tasks = vec![];

            for operator in operator_list.connected().await.clone() {
                let vse_setup = vse_setup.clone();
                let dkg_sessions = Arc::clone(&dkg_sessions);

                tasks.push(tokio::spawn(async move {
                    if let Ok(response) = operator
                        .request_dkg_packages(setup_no, num_sessions_to_fill)
                        .await
                    {
                        let dkg_sessions_ = {
                            let _dkg_sessions = dkg_sessions.lock().await;
                            (*_dkg_sessions).clone()
                        };

                        let operator_key = {
                            let _operator = operator.lock().await;
                            _operator.nns_key()
                        };

                        for (index, auth_package) in response.iter().enumerate() {
                            if auth_package.key() == operator_key {
                                if auth_package.authenticate() {
                                    let dkg_session = Arc::clone(&dkg_sessions_[index]);

                                    {
                                        let mut _dkg_session = dkg_session.lock().await;
                                        let _ = _dkg_session.insert(&auth_package, &vse_setup);
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
    }
}
