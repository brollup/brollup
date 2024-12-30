use crate::{
    tcp_client::{self, Request},
    vse, Peer, PeerList,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn run(operator_list: &PeerList) -> Option<vse::Directory> {
    eprintln!("Running VSE setup protocol.");

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

    let mut directory = vse::Directory::new(&connected_operator_key_list);

    for connected_operator in connected_operator_list {
        let map = match connected_operator
            .retrieve_vse_keymap(&connected_operator_key_list)
            .await
        {
            Ok(map) => map,
            Err(_) => continue,
        };

        if !directory.insert(map.clone()) {
            println!("Directory insertion failed.");
            return None;
        }

        println!("VSE keys retrieved from: {}", hex::encode(map.signer_key()));
    }

    return Some(directory);
}
