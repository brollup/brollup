use crate::{into::IntoScalar, schnorr, tcp::client::TCPClient, PEER};

pub async fn command(coordinator: &PEER, sk: [u8; 32], pk: [u8; 32]) {
    let sk_scalar = match sk.into_scalar() {
        Ok(scalar) => scalar,
        Err(_) => return,
    };

    let hiding_secret = match schnorr::generate_secret().into_scalar() {
        Ok(scalar) => scalar,
        Err(_) => return,
    };

    let hiding_public = hiding_secret.base_point_mul();

    let binding_secret = match schnorr::generate_secret().into_scalar() {
        Ok(scalar) => scalar,
        Err(_) => return,
    };

    let binding_public = binding_secret.base_point_mul();

    let partial_sig = match coordinator
        .cov_session_join(pk, hiding_public, binding_public)
        .await
    {
        Ok(musig_ctx) => {
            let agg_key = musig_ctx.key_agg_ctx().agg_key();
            let agg_nonce = match musig_ctx.agg_nonce() {
                Some(nonce) => nonce,
                None => {
                    println!("agg_nonce not found.");
                    return;
                }
            };

            println!("Agg key: {}", hex::encode(agg_key.serialize_xonly()));
            println!("Agg nonce: {}", hex::encode(agg_nonce.serialize_xonly()));

            let partial_sig = match musig_ctx.partial_sign(sk_scalar, hiding_secret, binding_secret)
            {
                Some(sig) => sig,
                None => {
                    println!("Error producing partial sig");
                    return;
                }
            };

            partial_sig
        }
        Err(_) => {
            eprintln!("Error joining signing session.");
            return;
        }
    };

    println!("Partial sig: {}", hex::encode(partial_sig.serialize()));

    if let Err(_) = coordinator.cov_session_submit(pk, partial_sig).await {
        eprintln!("Error submitting partial sig.");
    }
}
