use crate::{
    into::IntoPointVec, liquidity, noist::setup::setup::VSESetup, peer::PeerConnection,
    tcp::client::TCPClient, DKG_MANAGER, PEER, PEER_MANAGER,
};
use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub enum SignatorySetupError {
    PeerRetrievalErr,
    InsufficientPeers,
    PreSetupInitErr,
    PostSetupVerifyErr,
    ManagerInsertionErr,
}

pub async fn run_setup(
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &DKG_MANAGER,
) -> Result<VSESetup, SignatorySetupError> {
    // #1 Pick a setup number.
    let setup_no = {
        let _dkg_manager = dkg_manager.lock().await;
        _dkg_manager.setup_height() + 1
    };

    // #2 Retrieve the liquidity provider list.
    let lp_keys = liquidity::provider::provider_list();

    // #3 Connect to liquidity providers (if possible).
    let lp_peers: Vec<PEER> = match {
        let mut _peer_manager = peer_manager.lock().await;

        _peer_manager
            .add_peers(crate::peer::PeerKind::Operator, &lp_keys)
            .await;

        _peer_manager.retrieve_peers(&lp_keys)
    } {
        Some(some) => some,
        None => return Err(SignatorySetupError::PeerRetrievalErr),
    };

    // #4 Check if there are enough peer connections.
    if lp_peers.len() < lp_keys.len() / 10 {
        return Err(SignatorySetupError::InsufficientPeers);
    }

    // #5 Convert LP keys into secp Points.
    let lp_key_points = match lp_keys.into_point_vec() {
        Ok(points) => points,
        Err(_) => return Err(SignatorySetupError::PreSetupInitErr),
    };

    // #6 Initialize VSE setup with the list of LP keys.
    let vse_setup_ = match VSESetup::new(&lp_key_points, setup_no) {
        Some(setup) => Arc::new(Mutex::new(setup)),
        None => return Err(SignatorySetupError::PreSetupInitErr),
    };

    // #7 Retrieve VSE Keymap's from each connected LP peer.
    {
        let mut tasks = vec![];

        for lp_peer in lp_peers.clone() {
            let vse_setup_ = Arc::clone(&vse_setup_);
            let lp_keys = lp_keys.clone();

            let lp_key = lp_peer.key().await;

            tasks.push(tokio::spawn(async move {
                let keymap = match lp_peer.request_vse_keymap(&lp_keys).await {
                    Ok(keymap) => keymap,
                    Err(_) => return,
                };

                if keymap.signatory().serialize_xonly() == lp_key {
                    let mut _vse_setup_ = vse_setup_.lock().await;
                    _vse_setup_.insert_keymap(keymap);
                }
            }));
        }

        join_all(tasks).await;
    }

    // #8 Return the original VSE Setup struct.
    let mut vse_setup = {
        let _vse_setup = vse_setup_.lock().await;
        (*_vse_setup).clone()
    };

    // #9 Remove liquidity providers that failed to connect.
    vse_setup.remove_missing();

    // #10 Verify the final VSE setup.
    if !vse_setup.verify() {
        return Err(SignatorySetupError::PostSetupVerifyErr);
    };

    // #11 Deliver VSE setup to each connected liquidity provider.
    {
        let mut tasks = vec![];

        for lp_peer in lp_peers.clone() {
            let lp_key = lp_peer.key().await;

            if vse_setup.is_signatory(lp_key) {
                let vse_setup = vse_setup.clone();

                tasks.push(tokio::spawn(async move {
                    let _ = lp_peer.deliver_vse_setup(&vse_setup).await;
                }));
            }
        }

        join_all(tasks).await;
    }

    // #12 Insert VSE setup to local DKG directory.
    {
        let mut _dkg_manager = dkg_manager.lock().await;

        if !_dkg_manager.insert_setup(&vse_setup) {
            return Err(SignatorySetupError::ManagerInsertionErr);
        }
    };

    // #13 Return the VSE setup.
    Ok(vse_setup)
}
