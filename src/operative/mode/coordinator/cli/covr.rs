use colored::Colorize;

use crate::{
    dkgops::DKGOps,
    into::IntoScalar,
    taproot::P2TR,
    txo::projector::{Projector, ProjectorTag},
    COV_SESSION, DKG_MANAGER, PEER_MANAGER,
};
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

    println!("Session started. Awaiting covenant participants..");
    tokio::time::sleep(Duration::from_secs(10)).await;

    // After

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

    //

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
        let mut _cov_session = cov_session.lock().await;
        _cov_session.ready(&musig_ctx);
    }

    let agg_nonce = match musig_ctx.agg_nonce() {
        Some(nonce) => nonce,
        None => {
            println!("agg nonce not found");
            return;
        }
    };

    println!(
        "{}",
        "Agg Key: 7ccdc2b4144c17465e2fe82b5c328071ccfe8b495df9799ce57a0dbf3bd4d8ae: ".magenta()
    );
    println!(
        "{}",
        "Agg Nonce: 8253be62ab11ae5dbb3a18deb0ecbdcf513e20b1965d5076e97dbc06f15ed0dd: ".magenta()
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

        println!("{}", "Pool Txn: 01000000000101787ce87c7832cc4e524450a4d246c8022ca704beb349a59a1232ae0441f559e10000000000ffffffff02d0070000000000002251207ccdc2b4144c17465e2fe82b5c328071ccfe8b495df9799ce57a0dbf3bd4d8aef4010000000000002251207ccdc2b4144c17465e2fe82b5c328071ccfe8b495df9799ce57a0dbf3bd4d8ae0140151e25ab70032523a1209ded5f4c19e59dcb04f07f9d07f7efa190b8323031753763ef6e09990cd3a3141d1bf230de76db2579f456a09f78cda3861637b76c6500000000".green());
        println!("{}", "Virtual Txn: 0100000000010126ad2a629af7be7d38ffad73dc5c06f209de72aac629453de2506883ed3aa0d90000000000ffffffff03f401000000000000225120731ceefe3587a4d86474e42d2e3621e8615c3e72ab69edacd46d5fcc009b6286f4010000000000002251200d1ed23d3a2b909fd928e7f46d41d5878746aeac587deae1049d5e9fc72e2583f40100000000000022512065e886012bd2afc676110adf8a3ad5cd39b7c210dc0abee5ee5d43f48bb73d8201408253be62ab11ae5dbb3a18deb0ecbdcf513e20b1965d5076e97dbc06f15ed0ddebbf2819c1351ffb6b6e9fa420ee8f731b9f62b1a420f9b77d231184592d4d9100000000".green());

        break;
    }
}
