use crate::{key::KeyHolder, txo::lift::Lift, EPOCH_DIRECTORY};

pub async fn lifts_to_scan(key_holder: &KeyHolder, epoch_dir: &EPOCH_DIRECTORY) {
    let mut lifts = Vec::<Lift>::new();

    let self_key = key_holder.public_key();

    let group_keys = {
        let _epoch_dir = epoch_dir.lock().await;
        _epoch_dir.group_keys()
    };

    for group_key in group_keys.iter() {

    }
}
