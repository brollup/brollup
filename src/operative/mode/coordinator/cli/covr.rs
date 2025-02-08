use crate::{dkgops::DKGOps, into::IntoScalar, DKG_MANAGER, PEER_MANAGER, SESSION_CTX};
use std::time::Duration;

// covr <height> <msg>
pub async fn command(
    parts: Vec<&str>,
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &DKG_MANAGER,
    session_ctx: &mut SESSION_CTX,
) {
    let message = [0xffu8; 32];

    if parts.len() != 3 {
        eprintln!("Invalid usage.");
        return;
    }

    let dir_height = match parts[1].to_owned().parse::<u64>() {
        Ok(height) => height,
        Err(_) => return eprintln!("Invalid <height>."),
    };

    let msg: [u8; 32] = match hex::decode(parts[2]) {
        Ok(bytes) => match bytes.try_into() {
            Ok(msg) => msg,
            Err(_) => return eprintln!("Invalid <msg>."),
        },
        Err(_) => return eprintln!("Invalid <msg>."),
    };

    let dkg_dir = {
        let _dkg_manager = dkg_manager.lock().await;
        match _dkg_manager.directory(dir_height) {
            Some(dir) => dir,
            None => return eprintln!("Invalid <height>."),
        }
    };

    let operator_key = {
        let _dkg_dir = dkg_dir.lock().await;
        match _dkg_dir.group_key() {
            Some(key) => key,
            None => return eprintln!("Invalid <height>."),
        }
    };

    // open session
    {
        let mut _cov_session = session_ctx.lock().await;
        _cov_session.on();
    }

    println!("Session started. Awaiting covenant participants..");
    tokio::time::sleep(Duration::from_secs(10)).await;

    // After

    let remote = {
        let mut _session_ctx = session_ctx.lock().await;
        _session_ctx.remote()
    };

    if remote.len() == 0 {
        println!("No remote joined the session.");
        return;
    }

    for (index, (key, (hiding, binding))) in remote.iter().enumerate() {
        println!(
            "Remote #{} key: {}",
            index,
            hex::encode(key.serialize_xonly())
        );
        println!(
            "Remote #{} hiding nonce: {}",
            index,
            hex::encode(hiding.serialize_xonly())
        );
        println!(
            "Remote #{} binding nonce: {}",
            index,
            hex::encode(binding.serialize_xonly())
        );
        println!("");
    }

    // lock session
    {
        let mut _session_ctx = session_ctx.lock().await;
        _session_ctx.lock();
    }

    let noist_signing_session = {
        let mut dkg_dir_ = dkg_dir.lock().await;
        dkg_dir_.pick_signing_session(message, None, false).unwrap()
    };

    let nonce_index = noist_signing_session.nonce_index();

    let operator_key = noist_signing_session.group_key();
    let operator_hiding_nonce = noist_signing_session.hiding_group_nonce();
    let operator_binding_nonce = noist_signing_session.post_binding_group_nonce();

    let mut musig_ctx = match {
        let mut _session_ctx = session_ctx.lock().await;
        _session_ctx.set_musig_ctx(operator_key, operator_hiding_nonce, operator_binding_nonce)
    } {
        Some(ctx) => ctx,
        None => {
            eprintln!("Error returning musig nesting ctx.");
            return;
        }
    };

    //

    let messages = vec![(Some(nonce_index), msg, Some(musig_ctx.clone()))];

    let operator_partial_sig_ = match dkg_manager.sign(peer_manager, dir_height, messages).await {
        Ok(sig) => sig,
        Err(err) => {
            eprintln!("Error operator_partial_sig: {:?}", err);

            {
                let mut _session_ctx = session_ctx.lock().await;
                _session_ctx.reset();
            }

            return;
        }
    };

    let operator_partial_sig = operator_partial_sig_[0].clone();

    println!(
        "Agg key: {}",
        hex::encode(musig_ctx.key_agg_ctx().agg_key().serialize_xonly())
    );

    //

    let partial_sig_scalar = match (&operator_partial_sig[32..]).to_vec().into_scalar() {
        Ok(scalar) => scalar,
        Err(_) => return eprintln!("errr"),
    };

    if !musig_ctx.insert_partial_sig(operator_key, partial_sig_scalar) {
        eprintln!("Error insertirng operator partial sig.")
    }

    {
        let mut _session_ctx = session_ctx.lock().await;
        _session_ctx.ready(&musig_ctx);
    }

    loop {
        let _full_agg_sig = match {
            let _session_ctx = session_ctx.lock().await;
            _session_ctx.full_agg_sig()
        } {
            Some(sig) => sig,
            None => {
                tokio::time::sleep(Duration::from_millis(50)).await;
                continue;
            }
        };

        {
            let mut _session_ctx = session_ctx.lock().await;
            _session_ctx.finalized();
        }

        break;
    }
}
