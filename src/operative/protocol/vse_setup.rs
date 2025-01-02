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
    println!("beign");
    let (operators, keys) = {
        (
            operator_list.active_peers().await,
            operator_list.active_keys().await,
        )
    };

    println!("operators len: {}", operators.len());
    println!("keys len: {}", keys.len());

    let vse_setup: VSE_SETUP = {
        let setup_ = VSESetup::new(&keys);
        Arc::new(Mutex::new(setup_))
    };

    println!("ara 0");

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

            println!("{} a soruyoruz.", hex::encode(key));

            tasks.push(tokio::spawn(async move {
                let auth_keymap = match operator.request_vse_keymap(key, &keys).await {
                    Ok(auth_keymap) => auth_keymap,
                    Err(_) => return,
                };
                println!("{} dondu.", hex::encode(key));

                // Insertion.
                {
                    let mut _vse_setup = vse_setup.lock().await;

                    if !_vse_setup.insert(auth_keymap) {
                        println!("Insertion olmadi.");
                    } else {
                        println!("Insertion oldu.");
                    }
                }
            }));
        }

        join_all(tasks).await;
    }
    println!("ara 1");
    let vse_setup_ = {
        let _vse_setup = vse_setup.lock().await;
        (*_vse_setup).clone()
    };

    println!("ara 2");
    if !vse_setup_.validate() {
        return None;
    }
    println!("ara 3");
    let mut directory_ = {
        let mut _vse_directory = vse_directory.lock().await;
        (*_vse_directory).clone()
    };
    println!("ara 4");
    if !directory_.insert(no, &vse_setup_, signatory_db).await {
        return None;
    }
    println!("ara 5");
    // Directory is final.

    {
        let mut _vse_directory = vse_directory.lock().await;
        *_vse_directory = directory_.clone();
    }
    println!("ara 6");
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
    println!("ara 7");

    Some(vse_setup_)
}
