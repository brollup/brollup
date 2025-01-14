use crate::{
    into::IntoPointVec, liquidity, noist::setup::setup::VSESetup, tcp::client::TCPClient,
    DKG_MANAGER, PEER, PEER_MANAGER,
};
use colored::Colorize;
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

    let signatories = liquidity::provider::provider_list();

    // Check if the 'setup no' is already reserved.
    {
        let _dkg_manager = dkg_manager.lock().await;

        if let Some(_) = _dkg_manager.directory(setup_no) {
            eprintln!("{}", format!("Setup no is already reserved.").red());
            return None;
        }
    };

    // Connect to peers and return:
    let operators: Vec<PEER> = {
        let mut _peer_manager = peer_manager.lock().await;
        _peer_manager
            .add_peers(crate::peer::PeerKind::Operator, &signatories)
            .await;
        _peer_manager.retrieve_peers(&signatories)
    }?;

    let vse_setup = match VSESetup::new(&signatories.into_point_vec().ok()?, setup_no) {
        Some(setup) => Arc::new(Mutex::new(setup)),
        None => return None,
    };

    // Phase #1: Retrieve keymaps and insert setup.
    {
        let mut tasks = vec![];

        for operator in operators.clone() {
            let vse_setup = Arc::clone(&vse_setup);
            let signatories = signatories.clone();

            let operator_key = {
                let _connected_operator = operator.lock().await;
                _connected_operator.key()
            };

            tasks.push(tokio::spawn(async move {
                let keymap = match operator.request_vse_keymap(&signatories).await {
                    Ok(keymap) => keymap,
                    Err(_) => return,
                };

                if keymap.signatory().serialize_xonly() == operator_key {
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

    // Directory insertion.
    {
        let mut _dkg_manager = dkg_manager.lock().await;

        if !_dkg_manager.insert_setup(&vse_setup_) {
            return None;
        }
    };

    // Phase #2: Deliver setup to each operator.
    {
        let mut tasks = vec![];

        for operator in operators.clone() {
            let vse_setup_ = vse_setup_.clone();

            let operator_key = {
                let _operator = operator.lock().await;
                _operator.key()
            };

            tasks.push(tokio::spawn(async move {
                match operator.deliver_vse_setup(&vse_setup_).await {
                    Ok(_) => (),
                    Err(_) => eprintln!(
                        "{}",
                        format!("Failed to deliver: {}", hex::encode(&operator_key)).yellow()
                    ),
                }
            }));
        }

        join_all(tasks).await;
    }

    Some(vse_setup_)
}
