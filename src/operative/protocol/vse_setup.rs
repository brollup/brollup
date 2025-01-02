use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    tcp_client::Client, tcp_peer::PeerListExt, vse, PEER_LIST, SIGNATORY_DB, VSE_DIRECTORY,
    VSE_SETUP,
};

pub async fn run(
    operator_list: &PEER_LIST,
    signatory_db: &SIGNATORY_DB,
    vse_directory: &VSE_DIRECTORY,
    no: u64,
) -> Option<vse::Setup> {
    println!("beign");
    let (operators, keys) = {
        (
            operator_list.active_peers().await,
            operator_list.active_keys().await,
        )
    };

    println!("operators len: {}", operators.len());
    println!("keys len: {}", keys.len());

    let setup: VSE_SETUP = {
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

            println!("{} a soruyoruz.", hex::encode(key));

            tasks.push(tokio::spawn(async move {
                let auth_keymap = match operator.retrieve_vse_keymap(key, &keys).await {
                    Ok(auth_keymap) => auth_keymap,
                    Err(_) => return,
                };
                println!("{} dondu.", hex::encode(key));

                // Insertion.
                {
                    let mut _setup = setup.lock().await;

                    if !_setup.insert(auth_keymap) {
                        println!("Insertion da oldu.");
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

    println!("pre");
    if !setup_.validate() {
        return None;
    }
    println!("post");
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
