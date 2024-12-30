use crate::{
    tcp_client::{self, Request},
    vse, Peer, PeerList, VSEDirectory,
};
use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn run(operator_list: &PeerList) -> Option<VSEDirectory> {
    let mut connected_operator_key_list = Vec::<[u8; 32]>::new();
    let connected_operator_list: Vec<Peer> = {
        let mut list: Vec<Arc<Mutex<tcp_client::Peer>>> = Vec::<Peer>::new();
        let _operator_list = operator_list.lock().await;

        for (_, peer) in _operator_list.iter().enumerate() {
            let conn = {
                let _peer = peer.lock().await;
                _peer.connection()
            };

            if let Some(_) = conn {
                {
                    let _peer = peer.lock().await;
                    connected_operator_key_list.push(_peer.nns_key());
                }

                list.push(Arc::clone(&peer));
            }
        }
        list
    };

    let directory: VSEDirectory = {
        let directory = vse::Directory::new(&connected_operator_key_list);
        Arc::new(Mutex::new(directory))
    };

    let mut tasks = vec![];

    for connected_operator in connected_operator_list {
        let connected_operator_key_list = connected_operator_key_list.clone();
        let directory = Arc::clone(&directory);

        tasks.push(tokio::spawn(async move {
            let map = match connected_operator
                .retrieve_vse_keymap(&connected_operator_key_list)
                .await
            {
                Ok(map) => map,
                Err(_) => return,
            };

            let mut _directory = directory.lock().await;

            if !_directory.insert(map.clone()) {
                return;
            }
        }));
    }

    join_all(tasks).await;

    let valid = {
        let _directory = directory.lock().await;
        _directory.validate()
    };

    match valid {
        true => return Some(directory),
        false => return None,
    }
}
