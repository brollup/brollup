type LiftSPK = Vec<u8>;

use crate::{key::KeyHolder, taproot::P2TR, txo::lift::Lift, EPOCH_DIRECTORY};

/// Returns the list of Taproot scriptpubkeys to scan.
pub async fn lifts_spks_to_scan(
    key_holder: &KeyHolder,
    epoch_dir: &EPOCH_DIRECTORY,
) -> Option<Vec<LiftSPK>> {
    let mut spks: Vec<LiftSPK> = Vec::<LiftSPK>::new();

    let self_key = key_holder.public_key();

    let group_keys = {
        let _epoch_dir = epoch_dir.lock().await;
        _epoch_dir.group_keys()
    };

    for operator_group_key in group_keys.iter() {
        let lift = Lift::new(self_key, operator_group_key.to_owned(), None, None);
        let taproot = lift.taproot()?;
        let spk = taproot.spk()?;

        spks.push(spk);
    }

    Some(spks)
}
