#[cfg(test)]
mod noist_tests {
    use brollup::hash::Hash;
    use brollup::into::{IntoPoint, IntoPointVec, IntoScalar};
    use brollup::musig::MusigNestingCtx;
    use brollup::noist::dkg::package::DKGPackage;
    use brollup::noist::manager::DKGManager;
    use brollup::schnorr;
    use brollup::taproot::P2TR;
    use brollup::txn::{sigmsg_txn_1, sigmsg_txn_2, tx_1_build, tx_1_id, tx_2_build};
    use brollup::txo::projector::{Projector, ProjectorTag};
    use brollup::{
        noist::setup::{keymap::VSEKeyMap, setup::VSESetup},
        schnorr::Authenticable,
    };
    use secp::Point;
    use std::collections::HashMap;

    #[tokio::test]
    async fn noist_test() -> Result<(), String> {
        let signer_1_secret: [u8; 32] =
            hex::decode("396e7f3b89843e1e5610b1fdbaabf1b6a53066f43b22c529f839d69b6799ce8f")
                .unwrap()
                .try_into()
                .unwrap();
        let signer_1_public: [u8; 32] =
            hex::decode("eae0001e445c4f748f91010c1fb6d5b99391e588e605dbbb6ca4e5d98e520cd7")
                .unwrap()
                .try_into()
                .unwrap();

        let signer_2_secret: [u8; 32] =
            hex::decode("31dfea206f96e7b254e00fddb22baac233feb57d6ea98f3fe6929becad1eee78")
                .unwrap()
                .try_into()
                .unwrap();
        let signer_2_public: [u8; 32] =
            hex::decode("25451c1c2d326a14e86c7921cb1467512c944801c4fc0f81f8bd89e85d3ab1f1")
                .unwrap()
                .try_into()
                .unwrap();

        let signer_3_secret: [u8; 32] =
            hex::decode("38e2361ab771574909a9768670fa33406a311a2cae7d446359f09df18ac2cb83")
                .unwrap()
                .try_into()
                .unwrap();
        let signer_3_public: [u8; 32] =
            hex::decode("e8e5393d1873b616c12c6e2bee0c637f58dc5762dda654903c4dd1a72d762c34")
                .unwrap()
                .try_into()
                .unwrap();

        let signer_1_public = signer_1_public.into_point().unwrap();
        let signer_2_public = signer_2_public.into_point().unwrap();
        let signer_3_public = signer_3_public.into_point().unwrap();

        let mut public_list = vec![signer_1_public, signer_2_public, signer_3_public];
        public_list.sort();

        let manager_ = DKGManager::new().unwrap();
        let mut manager = manager_.lock().await;

        // Insert VSE setup to the manager.
        let setup_no: u64 = 0;

        // Signer 1 keymap.
        let signer_1_keymap = VSEKeyMap::new(signer_1_secret, &public_list).unwrap();
        assert!(signer_1_keymap.verify(&public_list));

        // Signer 2 keymap.
        let signer_2_keymap = VSEKeyMap::new(signer_2_secret, &public_list).unwrap();
        assert!(signer_2_keymap.verify(&public_list));

        // Signer 3 keymap.
        let signer_3_keymap = VSEKeyMap::new(signer_3_secret, &public_list).unwrap();
        assert!(signer_3_keymap.verify(&public_list));

        let mut vse_setup = VSESetup::new(&public_list, setup_no).unwrap();
        assert!(vse_setup.insert_keymap(signer_1_keymap));
        assert!(vse_setup.insert_keymap(signer_2_keymap));
        assert!(vse_setup.insert_keymap(signer_3_keymap));

        assert!(vse_setup.verify());

        manager.insert_setup(&vse_setup);

        // Retrieve DKG Directory from the manager.
        let mut dkg_directory = {
            let dir = manager.directory(setup_no).unwrap();
            let _dir = dir.lock().await;
            (*_dir).clone()
        };

        // populate directory with 10 DKG sessions: the first for the group key and the remaining 9 for the group nonce.
        for _ in 0..10 {
            let mut dkg_session = dkg_directory.new_session_to_fill().unwrap();

            // Communicate and fill.

            let s1_package = {
                let package = DKGPackage::new(signer_1_secret, &public_list).unwrap();
                Authenticable::new(package, signer_1_secret).unwrap()
            };
            let s2_package = {
                let package = DKGPackage::new(signer_2_secret, &public_list).unwrap();
                Authenticable::new(package, signer_2_secret).unwrap()
            };
            let s3_package = {
                let package = DKGPackage::new(signer_3_secret, &public_list).unwrap();
                Authenticable::new(package, signer_3_secret).unwrap()
            };

            if !dkg_session.insert(&s1_package, &vse_setup) {
                return Err("s1_package insertion err.".into());
            }
            if !dkg_session.insert(&s2_package, &vse_setup) {
                return Err("s2_package insertion err.".into());
            }
            if !dkg_session.insert(&s3_package, &vse_setup) {
                return Err("s3_package insertion err.".into());
            }
            if !dkg_session.verify(&vse_setup) {
                return Err("dkg_session verify err.".into());
            };

            if !dkg_directory.insert_session_filled(&dkg_session) {
                return Err("dkg_directory insertion err.".into());
            }
        }

        let group_key = dkg_directory.group_key().unwrap().serialize_xonly();

        // Sign 4 messages by consuming the 4 nonce sessions.
        for i in 0..4 {
            let message = format!("Signing our {}th joint message!", i)
                .as_bytes()
                .hash(None);

            let mut signing_session = dkg_directory.pick_signing_session(message, None).unwrap();

            let s1_partial_sig = signing_session.partial_sign(signer_1_secret).unwrap();
            let s2_partial_sig = signing_session.partial_sign(signer_2_secret).unwrap();
            let s3_partial_sig = signing_session.partial_sign(signer_3_secret).unwrap();

            if !signing_session.partial_sig_verify(signer_1_public, s1_partial_sig) {
                return Err("s1_partial_sig verify err.".into());
            };
            if !signing_session.partial_sig_verify(signer_2_public, s2_partial_sig) {
                return Err("s2_partial_sig verify err.".into());
            };
            if !signing_session.partial_sig_verify(signer_3_public, s3_partial_sig) {
                return Err("s3_partial_sig verify err.".into());
            };

            let agg_sig = match i {
                0 => {
                    // Case #1: only signer 1, and 2 produced.
                    if !signing_session.insert_partial_sig(signer_1_public, s1_partial_sig) {
                        return Err("s1_partial_sig insertion err.".into());
                    };
                    if !signing_session.insert_partial_sig(signer_2_public, s2_partial_sig) {
                        return Err("s2_partial_sig insertion err.".into());
                    };
                    signing_session.full_aggregated_sig_bytes().unwrap()
                }
                1 => {
                    // Case #2: only signer 1, and 3 produced.
                    if !signing_session.insert_partial_sig(signer_1_public, s1_partial_sig) {
                        return Err("s1_partial_sig insertion err.".into());
                    };
                    if !signing_session.insert_partial_sig(signer_3_public, s3_partial_sig) {
                        return Err("s3_partial_sig insertion err.".into());
                    };
                    signing_session.full_aggregated_sig_bytes().unwrap()
                }
                2 => {
                    // Case #3: only signer 2, and 3 produced.
                    if !signing_session.insert_partial_sig(signer_2_public, s2_partial_sig) {
                        return Err("s2_partial_sig insertion err.".into());
                    };
                    if !signing_session.insert_partial_sig(signer_3_public, s3_partial_sig) {
                        return Err("s3_partial_sig insertion err.".into());
                    };
                    signing_session.full_aggregated_sig_bytes().unwrap()
                }
                3 => {
                    // Case #4: all signers 1, 2, and 3 produced.
                    if !signing_session.insert_partial_sig(signer_1_public, s1_partial_sig) {
                        return Err("s1_partial_sig insertion err.".into());
                    };
                    if !signing_session.insert_partial_sig(signer_2_public, s2_partial_sig) {
                        return Err("s2_partial_sig insertion err.".into());
                    };
                    if !signing_session.insert_partial_sig(signer_3_public, s3_partial_sig) {
                        return Err("s3_partial_sig insertion err.".into());
                    };
                    signing_session.full_aggregated_sig_bytes().unwrap()
                }
                _ => [0xffu8; 64],
            };

            if !schnorr::verify(group_key, message, agg_sig, schnorr::SigningMode::BIP340) {
                return Err("Invalid aggregate schnorr signature.".into());
            }
        }

        // Musig test.

        //let message = format!("MUassdfdSIG!").as_bytes().hash(None);

        let musig_signer_secret: [u8; 32] =
            hex::decode("c010e9eb7ccfbd1c6e742f85100936c9996e5e0d6255ec1162a8e85653e286d9")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer_public: [u8; 33] =
            hex::decode("031b7e3d01a7dcbab38d722b003e4aea1f8d3889469848230cb9b1ad51f91cb598")
                .unwrap()
                .try_into()
                .unwrap();

        let musig_signer_hiding_secret_nonce: [u8; 32] =
            hex::decode("2bf1ce5c2e57b5080e3d67e77b0a015919dc55fd44e30d65f7817d8afd6995c6")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer_hiding_public_nonce: [u8; 33] =
            hex::decode("02b8e828457c6ab70e3f8a9cfdd5c21172fa69fccfeb74ec5e7a784f272ed094d6")
                .unwrap()
                .try_into()
                .unwrap();

        let musig_signer_binding_secret_nonce: [u8; 32] =
            hex::decode("320b845c8ee898fd8f53ae2eacab49f0d244e9ac0ab94f5cfb2098ec654505c2")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer_binding_public_nonce: [u8; 33] =
            hex::decode("027923dc97eec709f6dfc56cac3eb59b5fbd47a89554202c79d9588a6dd37814ef")
                .unwrap()
                .try_into()
                .unwrap();

        let musig_signer2_secret: [u8; 32] =
            hex::decode("5a788716adc0bc641d79ce0ab90295bcdc5027c5d87cc29f8be02651b60e4cc1")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer2_public: [u8; 33] =
            hex::decode("03b63e209e5ddd84e402ec6833100a2c59f72fb7dd17df4a9616d39f25a060dd36")
                .unwrap()
                .try_into()
                .unwrap();

        let musig_signer2_hiding_secret_nonce: [u8; 32] =
            hex::decode("e0f530415cecfdae3105218d5c153c14e20a5faa2e24eb5ae11051015a48f77b")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer2_hiding_public_nonce: [u8; 33] =
            hex::decode("023206ce056a69d097b3bf511ee15689babb11586518f78e29b33b48986c78e1c1")
                .unwrap()
                .try_into()
                .unwrap();

        let musig_signer2_binding_secret_nonce: [u8; 32] =
            hex::decode("e87330abaeb33e6c7f757aae7267c5e666c7d2d38ca09e27db6371aa13b2c54b")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer2_binding_public_nonce: [u8; 33] =
            hex::decode("02571a8ceb8799f3a72685dd91d620336b2c0fe29d6e026de68257f67b539f15f0")
                .unwrap()
                .try_into()
                .unwrap();

        let musig_signer3_secret: [u8; 32] =
            hex::decode("40a2a99efa892c287d7aa0b2ba4eb005e05e5fd51980fd1ff35a3d375f379dec")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer3_public: [u8; 33] =
            hex::decode("035dc1e9abecc4623bbe4571d7a584280a84b19a59ebdae52cd51dfeeac774a5c8")
                .unwrap()
                .try_into()
                .unwrap();

        let musig_signer3_hiding_secret_nonce: [u8; 32] =
            hex::decode("d339d0fd55e80a656676a2b9798367762f82e36fd5c71dd8bbcde9b5c3fb923d")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer3_hiding_public_nonce: [u8; 33] =
            hex::decode("03b8c8f30aef38216eed131e2aa37a060e18945e3372d3b85d7c548f02879fa1f6")
                .unwrap()
                .try_into()
                .unwrap();

        let musig_signer3_binding_secret_nonce: [u8; 32] =
            hex::decode("0e95284da81c53594cef642da41eb8d81637f4f1e854cef7d578ffa2ba32b9e6")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer3_binding_public_nonce: [u8; 33] =
            hex::decode("03792a087c86671ebd5e52e3c64cd992a7f4f43bead3aa8177fe3324af49e51a51")
                .unwrap()
                .try_into()
                .unwrap();

        let remote_keys = vec![
            musig_signer_public,
            musig_signer2_public,
            musig_signer3_public,
        ]
        .into_point_vec()
        .unwrap();
        let operator_key = dkg_directory.group_key().unwrap();

        let projector = Projector::new(remote_keys, operator_key, ProjectorTag::VTXOProjector);
        let projector_txo = projector.taproot().unwrap();

        println!(
            "projector_txo spk: {}",
            hex::encode(projector_txo.spk().unwrap())
        );

        let spender_secret_key: [u8; 32] =
            hex::decode("09bc6b39594512478a62382cf64b2e8b7e5c0521b8d227b6430c61ca98449dd7")
                .unwrap()
                .try_into()
                .unwrap();
        let spender_public_key: [u8; 33] =
            hex::decode("0251d04ada8c0fb417581ab37da02e8e9439b51f0eb2300b722d532a73b7485e0e")
                .unwrap()
                .try_into()
                .unwrap();
        //bc1p28gy4k5vp76pwkq6kd76qt5wjsum28cwkgcqku3d2v488d6gtc8qyxtx4j

        // outpoint

        let prev_spk =
            hex::decode("512051d04ada8c0fb417581ab37da02e8e9439b51f0eb2300b722d532a73b7485e0e")
                .unwrap();

        let outpoint: [u8; 36] =
            hex::decode("f7a9b4a09e6b3dee941ced4a2e4855c8f5cd46a680b243d516bc570d071c06de00000000")
                .unwrap()
                .try_into()
                .unwrap();

        let txn_1_sigmsg = sigmsg_txn_1(outpoint, prev_spk, projector.clone()).unwrap();

        println!("txn_1_sigmsg : {}", hex::encode(txn_1_sigmsg));

        let sig = schnorr::sign(
            spender_secret_key,
            txn_1_sigmsg,
            schnorr::SigningMode::BIP340,
        )
        .unwrap();

        println!("sig : {}", hex::encode(sig));

        let txn_1 = tx_1_build(outpoint, projector.clone(), sig).unwrap();

        println!("txn_1 : {}", hex::encode(txn_1));

        let tx_1_id = tx_1_id(outpoint, projector.clone()).unwrap();

        println!("tx_1_id: {}", hex::encode(tx_1_id));

        let prev_spk = projector_txo.spk().unwrap();

        let key_1 = musig_signer_public.into_point().unwrap().serialize_xonly();
        let key_2 = musig_signer2_public.into_point().unwrap().serialize_xonly();
        let key_3 = musig_signer3_public.into_point().unwrap().serialize_xonly();

        let txn_2_sigmsg = sigmsg_txn_2(tx_1_id, prev_spk, key_1, key_2, key_3).unwrap();

        println!("txn_2_sigmsg : {}", hex::encode(txn_2_sigmsg));

        println!("operator_key: {}", hex::encode(operator_key.serialize()));

        let mut signers = HashMap::<Point, (Point, Point)>::new();
        signers.insert(
            musig_signer_public.into_point().unwrap(),
            (
                musig_signer_hiding_public_nonce.into_point().unwrap(),
                musig_signer_binding_public_nonce.into_point().unwrap(),
            ),
        );

        signers.insert(
            musig_signer2_public.into_point().unwrap(),
            (
                musig_signer2_hiding_public_nonce.into_point().unwrap(),
                musig_signer2_binding_public_nonce.into_point().unwrap(),
            ),
        );

        signers.insert(
            musig_signer3_public.into_point().unwrap(),
            (
                musig_signer3_hiding_public_nonce.into_point().unwrap(),
                musig_signer3_binding_public_nonce.into_point().unwrap(),
            ),
        );

        let tweak = projector_txo.tap_tweak().into_scalar().unwrap();

        let musig_nesting_ctx = MusigNestingCtx::new(signers, Some(tweak));

        let mut signing_session = dkg_directory
            .pick_signing_session(txn_2_sigmsg, Some(musig_nesting_ctx))
            .unwrap();

        let mut musig_ctx = signing_session.musig_ctx().unwrap();

        let agg_key = musig_ctx.agg_key();

        println!("agg_key: {}", hex::encode(agg_key.serialize()));

        let s1_partial_sig = signing_session.partial_sign(signer_1_secret).unwrap();
        let s2_partial_sig = signing_session.partial_sign(signer_2_secret).unwrap();

        if !signing_session.insert_partial_sig(signer_1_public, s1_partial_sig) {
            return Err("m s1_partial_sig insertion err.".into());
        };
        if !signing_session.insert_partial_sig(signer_2_public, s2_partial_sig) {
            return Err("m s2_partial_sig insertion err.".into());
        };

        let op_partial_sig = signing_session.aggregated_sig().unwrap();

        if !musig_ctx.insert_partial_sig(operator_key, op_partial_sig) {
            println!("musig_ctx op sig insert err.");
        }

        let client1_partial_sig = musig_ctx
            .partial_sign(
                musig_signer_public.into_point().unwrap(),
                musig_signer_secret.into_scalar().unwrap(),
                musig_signer_hiding_secret_nonce.into_scalar().unwrap(),
                musig_signer_binding_secret_nonce.into_scalar().unwrap(),
            )
            .unwrap();

        if !musig_ctx.insert_partial_sig(
            musig_signer_public.into_point().unwrap(),
            client1_partial_sig,
        ) {
            println!("client1 op sig insert err.");
        }

        let client2_partial_sig = musig_ctx
            .partial_sign(
                musig_signer2_public.into_point().unwrap(),
                musig_signer2_secret.into_scalar().unwrap(),
                musig_signer2_hiding_secret_nonce.into_scalar().unwrap(),
                musig_signer2_binding_secret_nonce.into_scalar().unwrap(),
            )
            .unwrap();

        if !musig_ctx.insert_partial_sig(
            musig_signer2_public.into_point().unwrap(),
            client2_partial_sig,
        ) {
            println!("client2 op sig insert err.");
        }

        let client3_partial_sig = musig_ctx
            .partial_sign(
                musig_signer3_public.into_point().unwrap(),
                musig_signer3_secret.into_scalar().unwrap(),
                musig_signer3_hiding_secret_nonce.into_scalar().unwrap(),
                musig_signer3_binding_secret_nonce.into_scalar().unwrap(),
            )
            .unwrap();

        if !musig_ctx.insert_partial_sig(
            musig_signer3_public.into_point().unwrap(),
            client3_partial_sig,
        ) {
            println!("client3 op sig insert err.");
        }

        let agg_sig = musig_ctx.full_agg_sig().unwrap();

        assert!(schnorr::verify(
            musig_ctx.agg_key().serialize_xonly(),
            txn_2_sigmsg,
            agg_sig,
            schnorr::SigningMode::BIP340
        ));

        println!("agg_sig: {}", hex::encode(agg_sig));
        let txn_2 = tx_2_build(tx_1_id, key_1, key_2, key_3, agg_sig).unwrap();

        println!("txn_2: {}", hex::encode(txn_2));

        Ok(())
    }
}
