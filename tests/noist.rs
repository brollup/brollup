#[cfg(test)]
mod noist_tests {
    use brollup::hash::Hash;
    use brollup::into::{IntoPoint, IntoScalar};
    use brollup::musig::MusigNestingCtx;
    use brollup::noist::dkg::package::DKGPackage;
    use brollup::noist::manager::DKGManager;
    use brollup::schnorr;
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

        // populate directory with 5 DKG sessions: the first for the group key and the remaining 4 for the group nonce.
        for _ in 0..5 {
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

        let message = format!("MUassdfdSIG!").as_bytes().hash(None);

        let musig_signer_secret: [u8; 32] =
            hex::decode("c0e10f188b0e93b67a5c1ec9fe15389997e2ea000555725cd74bd61af6faec4e")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer_public: [u8; 33] =
            hex::decode("0209d9f274df52d894d64a3360bb0d42cbc8783a60c1a719d5d8f5974918c01e16")
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
            hex::decode("609d4ccf1f15f8b4b9a4a77a9a55550e47036a32babf1cd138f3411a52b10b67")
                .unwrap()
                .try_into()
                .unwrap();
        let musig_signer2_public: [u8; 33] =
            hex::decode("02fd6520c13244fb8c412fca4b02a2d9a8a6062855c0dbf8123519180c9f271702")
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

        let tap_branch = [0xfe; 32];

        let musig_nesting_ctx = MusigNestingCtx::new(signers, Some(tap_branch));

        let mut signing_session = dkg_directory
            .pick_signing_session(message, Some(musig_nesting_ctx))
            .unwrap();

        let operator_key = signing_session.group_key;

        let mut musig_ctx = signing_session.musig_ctx().unwrap();

        let agg_key = musig_ctx.agg_key;

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
            println!("client op sig insert err.");
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
            println!("client op sig insert err.");
        }

        let agg_sig = musig_ctx.full_agg_sig().unwrap();

        let verify_key = match musig_ctx.tweaked_agg_key {
            Some(key) => key,
            None => agg_key,
        };

        assert!(schnorr::verify(
            verify_key.serialize_xonly(),
            message,
            agg_sig,
            schnorr::SigningMode::BIP340
        ));

        Ok(())
    }
}
