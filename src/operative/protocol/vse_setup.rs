use crate::{
    tcp_client::{PeerListExt, Request},
    vse, PeerList, SignatoryDB, VSEDirectory, VSESetup,
};
use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn run(
    operator_list: &PeerList,
    signatory_db: &SignatoryDB,
    vse_directory: &VSEDirectory,
    no: u64,
) -> Option<vse::Setup> {
    let (operators, keys) = {
        (
            operator_list.active_peers().await,
            operator_list.active_keys().await,
        )
    };

    let setup: VSESetup = {
        let setup_ = vse::Setup::new(&keys);
        Arc::new(Mutex::new(setup_))
    };

    // Phase #1: Retrieve keymaps and insert setup.
    {
        let mut tasks = vec![];

        for operator in operators.clone() {
            let keys = keys.clone();
            let setup = Arc::clone(&setup);

            let key = {
                let _connected_operator = operator.lock().await;
                _connected_operator.nns_key()
            };

            tasks.push(tokio::spawn(async move {
                let auth_keymap = match operator.retrieve_vse_keymap(key, &keys).await {
                    Ok(auth_keymap) => auth_keymap,
                    Err(_) => return,
                };

                // Insertion.
                {
                    let mut _setup = setup.lock().await;

                    if !_setup.insert(auth_keymap) {
                        return;
                    }
                }
            }));
        }

        join_all(tasks).await;
    }

    let setup_ = {
        let _setup = setup.lock().await;
        (*_setup).clone()
    };

    if !setup_.validate() {
        return None;
    }

    let mut directory_ = {
        let mut _vse_directory = vse_directory.lock().await;
        (*_vse_directory).clone()
    };

    if !directory_.insert(no, &setup_, signatory_db).await {
        return None;
    }

    // Directory is final.

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

    Some(setup_)
}
