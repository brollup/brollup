use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    noist::vse::VSESetup,
    tcp::{client::TCPClient, peer::PeerListExt},
    PEER_LIST, SIGNATORY_DB, VSE_DIRECTORY, VSE_SETUP,
};

pub async fn run(
    operator_list: &PEER_LIST,
    signatory_db: &SIGNATORY_DB,
    vse_directory: &VSE_DIRECTORY,
    no: u64,
) -> Option<VSESetup> {
    let (operators, keys) = {
        (
            operator_list.active_peers().await,
            operator_list.active_keys().await,
        )
    };

    let vse_setup: VSE_SETUP = {
        let setup_ = VSESetup::new(&keys);
        Arc::new(Mutex::new(setup_))
    };

    // Phase #1: Retrieve keymaps and insert setup.
    {
        let mut tasks = vec![];

        for operator in operators.clone() {
            let keys = keys.clone();
            let vse_setup: VSE_SETUP = Arc::clone(&vse_setup);

            let key = {
                let _connected_operator = operator.lock().await;
                _connected_operator.nns_key()
            };

            tasks.push(tokio::spawn(async move {
                let auth_keymap = match operator.request_vse_keymap(key, &keys).await {
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

    let mut directory_ = {
        let mut _vse_directory = vse_directory.lock().await;
        (*_vse_directory).clone()
    };

    if !directory_.insert(no, &vse_setup_, signatory_db).await {
        return None;
    }

    // Directory is ready.

    {
        let mut _vse_directory = vse_directory.lock().await;
        *_vse_directory = directory_.clone();
    }

    // Phase #2: Deliver directory to each operator.
    {
        let mut tasks = vec![];

        for operator in operators.clone() {
            let directory_ = directory_.clone();

            tasks.push(tokio::spawn(async move {
                operator.deliver_vse_directory(&directory_).await
            }));
        }

        join_all(tasks).await;
    }

    Some(vse_setup_)
}
