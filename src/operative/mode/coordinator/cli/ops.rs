use crate::PEER_MANAGER;

pub async fn command(peer_manager: &PEER_MANAGER) {
    let peers = {
        let _peer_manager = peer_manager.lock().await;
        _peer_manager.peers()
    };

    match peers.len() {
        0 => {
            println!("None.");
            return;
        }
        _ => {
            for (index, (key, peer)) in peers.iter().enumerate() {
                let _peer = peer.lock().await;
                println!(
                    "Operator #{} ({}): {}",
                    index,
                    hex::encode(key),
                    _peer.addr()
                );
            }
        }
    }
}
