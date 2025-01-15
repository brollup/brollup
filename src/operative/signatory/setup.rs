use crate::{
    into::IntoPointVec, liquidity, noist::setup::setup::VSESetup, tcp::client::TCPClient,
    DKG_MANAGER, PEER, PEER_MANAGER,
};
use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn run_setup(
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &DKG_MANAGER,
) -> Option<VSESetup> {
    let setup_no = {
        let _dkg_manager = dkg_manager.lock().await;
        _dkg_manager.setup_height() + 1
    };

    let lp_keys = liquidity::provider::provider_list();

    // Connect to peers and return:
    let lp_peers: Vec<PEER> = {
        let mut _peer_manager = peer_manager.lock().await;
        _peer_manager
            .add_peers(crate::peer::PeerKind::Operator, &lp_keys)
            .await;
        _peer_manager.retrieve_peers(&lp_keys)
    }?;

    let vse_setup = match VSESetup::new(&lp_keys.into_point_vec().ok()?, setup_no) {
        Some(setup) => Arc::new(Mutex::new(setup)),
        None => return None,
    };

    // Phase #1: Retrieve keymaps and insert setup.
    {
        let mut tasks = vec![];

        for lp_peer in lp_peers.clone() {
            let vse_setup = Arc::clone(&vse_setup);
            let lp_keys = lp_keys.clone();

            let lp_key = {
                let _lp_peer = lp_peer.lock().await;
                _lp_peer.key()
            };

            tasks.push(tokio::spawn(async move {
                let keymap = match lp_peer.request_vse_keymap(&lp_keys).await {
                    Ok(keymap) => keymap,
                    Err(_) => return,
                };

                if keymap.signatory().serialize_xonly() == lp_key {
                    let mut _vse_setup = vse_setup.lock().await;
                    _vse_setup.insert_keymap(keymap);
                }
            }));
        }

        join_all(tasks).await;
    }

    let mut vse_setup_ = {
        let _vse_setup = vse_setup.lock().await;
        (*_vse_setup).clone()
    };

    vse_setup_.remove_missing();

    if !vse_setup_.verify() {
        return None;
    };

    // Phase #2: Deliver setup to each operator.
    {
        let mut tasks = vec![];

        for lp_peer in lp_peers.clone() {
            let vse_setup_ = vse_setup_.clone();

            tasks.push(tokio::spawn(async move {
                let _ = lp_peer.deliver_vse_setup(&vse_setup_).await;
            }));
        }

        join_all(tasks).await;
    }

    // Directory insertion.
    {
        let mut _dkg_manager = dkg_manager.lock().await;

        if !_dkg_manager.insert_setup(&vse_setup_) {
            return None;
        }
    };

    Some(vse_setup_)
}
