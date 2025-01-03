use colored::Colorize;
use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    noist::vse::setup::VSESetup,
    tcp::{client::TCPClient, peer::PeerListExt},
    PEER, PEER_LIST, SIGNATORY_DB, VSE_DIRECTORY, VSE_SETUP,
};

pub async fn run(
    operator_list: &PEER_LIST,
    signatory_db: &SIGNATORY_DB,
    vse_directory: &VSE_DIRECTORY,
    no: u64,
) -> Option<VSESetup> {
    let no_reserved = {
        let _vse_directory = vse_directory.lock().await;
        _vse_directory.no_reserved(no)
    };

    if no_reserved {
        eprintln!("{}", format!("Setup no is already reserved.").red());
        return None;
    }

    let (active_peer_list_, active_key_list_) = {
        let active_peer_list = Vec::<PEER>::new();
        let active_key_list = Vec::<[u8; 32]>::new();

        (
            Arc::new(Mutex::new(active_peer_list)),
            Arc::new(Mutex::new(active_key_list)),
        )
    };

    // Phase #0: Ping operators to determine those who are online;
    {
        let mut tasks = vec![];

        for operator in operator_list.connected().await.clone() {
            let active_peer_list_ = Arc::clone(&active_peer_list_);
            let active_key_list_ = Arc::clone(&active_key_list_);

            let operator_nns_key = {
                let _operator = operator.lock().await;
                _operator.nns_key()
            };

            tasks.push(tokio::spawn(async move {
                if let Ok(_) = operator.ping().await {
                    let mut _active_peer_list_ = active_peer_list_.lock().await;
                    _active_peer_list_.push(Arc::clone(&operator));

                    let mut _active_key_list_ = active_key_list_.lock().await;
                    _active_key_list_.push(operator_nns_key);
                }
            }));
        }

        join_all(tasks).await;
    }

    let (active_peers, active_keys) = {
        let active_peer_list = {
            let _active_peer_list_ = active_peer_list_.lock().await;
            (*_active_peer_list_).clone()
        };

        let active_key_list = {
            let _active_key_list_ = active_key_list_.lock().await;
            (*_active_key_list_).clone()
        };
        (active_peer_list, active_key_list)
    };

    if active_keys.len() < 2 {
        eprintln!("{}", format!("Too few active operators.").red());
        return None;
    }

    let vse_setup: VSE_SETUP = {
        let setup_ = VSESetup::new(&active_keys, no);
        Arc::new(Mutex::new(setup_))
    };

    // Phase #1: Retrieve keymaps and insert setup.
    {
        let mut tasks = vec![];

        for operator in active_peers.clone() {
            let vse_setup: VSE_SETUP = Arc::clone(&vse_setup);
            let active_keys = active_keys.clone();

            let operator_key = {
                let _connected_operator = operator.lock().await;
                _connected_operator.nns_key()
            };

            tasks.push(tokio::spawn(async move {
                let auth_keymap = match operator
                    .request_vse_keymap(operator_key, &active_keys)
                    .await
                {
                    Ok(auth_keymap) => auth_keymap,
                    Err(_) => return,
                };

                // Insertion.
                {
                    let mut _vse_setup = vse_setup.lock().await;
                    _vse_setup.insert(auth_keymap);
                }
            }));
        }

        join_all(tasks).await;
    }

    let vse_setup_ = {
        let _vse_setup = vse_setup.lock().await;
        (*_vse_setup).clone()
    };

    if !vse_setup_.validate() {
        return None;
    }

    let directory_insertion = {
        let mut _vse_directory = vse_directory.lock().await;
        _vse_directory.insert(&vse_setup_, signatory_db).await
    };

    if !directory_insertion {
        return None;
    }

    // Phase #2: Deliver setup to each operator.
    {
        let mut tasks = vec![];

        for operator in active_peers.clone() {
            let vse_setup_ = vse_setup_.clone();

            let operator_key = {
                let _operator = operator.lock().await;
                _operator.nns_key()
            };

            tasks.push(tokio::spawn(async move {
                match operator.deliver_vse_setup(&vse_setup_).await {
                    Ok(_) => (),
                    Err(_) => eprintln!(
                        "{}",
                        format!("Failed to deliver: {}", hex::encode(&operator_key)).yellow()
                    ),
                }
            }));
        }

        join_all(tasks).await;
    }

    Some(vse_setup_)
}
