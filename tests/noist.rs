#[cfg(test)]
mod noist_tests {
    use brollup::hash::Hash;
    use brollup::into::IntoPoint;
    use brollup::noist::dkg::package::DKGPackage;
    use brollup::noist::manager::DKGManager;
    use brollup::schnorr;
    use brollup::{
        noist::setup::{keymap::VSEKeyMap, setup::VSESetup},
        schnorr::Authenticable,
    };

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

        // Sign 4 messahes by consuming the 4 nonce sessions.
        for i in 0..4 {
            let message = format!("Signing our {}th joint message!", i)
                .as_bytes()
                .hash(None);

            let mut signing_session = dkg_directory.pick_signing_session(message).unwrap();

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

        Ok(())
    }
}
