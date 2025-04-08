use crate::{
    constructive::{taproot::P2TR, txo::lift::Lift},
    inscriptive::epoch::dir::EPOCH_DIRECTORY,
    operative::Chain,
    transmutive::{codec::address::encode_p2tr, key::KeyHolder},
};

/// Returns the list of Taproot scriptpubkeys to scan.
pub async fn lift_address(
    chain: Chain,
    key_holder: &KeyHolder,
    epoch_dir: &EPOCH_DIRECTORY,
) -> Option<String> {
    let self_key = key_holder.public_key();

    let latest_active_epoch = {
        let _epoch_dir = epoch_dir.lock().await;
        _epoch_dir.latest_active_epoch()?
    };

    let lift = Lift::new(self_key, latest_active_epoch.group_key(), None, None);
    let taproot = lift.taproot()?;
    let taproot_key_bytes = taproot.tweaked_key()?.serialize_xonly();

    let address = encode_p2tr(chain, taproot_key_bytes)?;

    Some(address)
}

// addr
pub async fn addr_command(chain: Chain, epoch_dir: &EPOCH_DIRECTORY, key_holder: &KeyHolder) {
    let npub = key_holder.npub();
    let lift_address = match lift_address(chain, key_holder, epoch_dir).await {
        Some(address) => address,
        None => "-".to_string(),
    };

    println!("off-chain : {}\non-chain  : {}", npub, lift_address);
}
