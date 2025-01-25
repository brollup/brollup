use colored::Colorize;

use crate::{dkgops::DKGOps, into::IntoScalar, COV_SESSION, DKG_MANAGER, PEER_MANAGER};
use std::time::Duration;

// covr <height> <msg>
pub async fn command(
    parts: Vec<&str>,
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &DKG_MANAGER,
    cov_session: &mut COV_SESSION,
) {
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
        let mut _cov_session = cov_session.lock().await;
        _cov_session.on();
    }

    println!("Session started. Awaiting signers..");
    tokio::time::sleep(Duration::from_secs(10)).await;

    let remote = {
        let mut _cov_session = cov_session.lock().await;
        _cov_session.remote()
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
        let mut _cov_session = cov_session.lock().await;
        _cov_session.lock();
    }

    let musig_nesting_ctx = match {
        let mut _cov_session = cov_session.lock().await;
        _cov_session.musig_nesting_ctx()
    } {
        Some(ctx) => ctx,
        None => {
            eprintln!("Error returning musig nesting ctx.");
            return;
        }
    };

    let messages = vec![(msg, Some(musig_nesting_ctx))];

    let operator_partial_sig_ = match dkg_manager.sign(peer_manager, dir_height, messages).await {
        Ok(sig) => sig,
        Err(err) => {
            eprintln!("Error operator_partial_sig: {:?}", err);

            {
                let mut _cov_session = cov_session.lock().await;
                _cov_session.reset();
            }

            return;
        }
    };

    let (operator_partial_sig, musig_ctx) = operator_partial_sig_[0].clone();

    let mut musig_ctx = match musig_ctx {
        Some(ctx) => ctx,
        None => {
            eprintln!("Error musig_ctx.");
            return;
        }
    };

    let partial_sig_scalar = match (&operator_partial_sig[32..]).to_vec().into_scalar() {
        Ok(scalar) => scalar,
        Err(_) => return eprintln!("errr"),
    };

    if !musig_ctx.insert_partial_sig(operator_key, partial_sig_scalar) {
        eprintln!("Error insertirng operator partial sig.")
    }

    {
        let mut _cov_session = cov_session.lock().await;
        _cov_session.ready(&musig_ctx);
    }

    println!(
        "Agg key: {}",
        hex::encode(musig_ctx.agg_key().serialize_xonly())
    );

    println!(
        "Agg nonce: {}",
        hex::encode(musig_ctx.agg_nonce().serialize_xonly())
    );

    loop {
        let full_agg_sig = match {
            let _cov_session = cov_session.lock().await;
            _cov_session.full_agg_sig()
        } {
            Some(sig) => sig,
            None => {
                tokio::time::sleep(Duration::from_millis(50)).await;
                continue;
            }
        };

        {
            let mut _cov_session = cov_session.lock().await;
            _cov_session.finalized();
        }
        println!(
            "{}",
            format!("Agg sig: {}", hex::encode(full_agg_sig)).green()
        );
        break;
    }
}
