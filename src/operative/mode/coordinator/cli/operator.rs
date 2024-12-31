use crate::PeerList;

pub async fn command(operator_list: &PeerList) {
    let _operator_list = operator_list.lock().await;

    for (index, peer) in _operator_list.iter().enumerate() {
        let _peer = peer.lock().await;
        println!(
            "Operator #{} ({}): {}",
            index,
            hex::encode(_peer.nns_key()),
            _peer.addr()
        );
    }
}