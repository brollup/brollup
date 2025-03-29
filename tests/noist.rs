#[cfg(test)]
mod noist_tests {
    use brollup::{
        constructive::txo::projector::{Projector, ProjectorTag},
        inscriptive::lp::dir::LPDirectory,
        operative::Chain,
        transmutive::{
            hash::Hash,
            musig::session::MusigSessionCtx,
            noist::{
                dkg::package::DKGPackage,
                manager::DKGManager,
                setup::{keymap::VSEKeyMap, setup::VSESetup},
            },
            schnorr::{self, Authenticable},
        },
    };
    use secp::{Point, Scalar};

    #[tokio::test]
    async fn noist_test() -> Result<(), String> {
        let signer_1_secret =
            Scalar::from_hex("4dd13a95c4c55dd7fc2f058a12e56e702879c37a7641943f5a83b9d90c0fa37e")
                .unwrap();

        let signer_1_public =
            Point::from_hex("0244b24c49f6fbb510b9d48283f86207d3d23a9116b4c2c9d04c900d63a699512b")
                .unwrap();

        let signer_2_secret =
            Scalar::from_hex("a3f1048fae9a97776810160dfb5221649bfe3604ab3a3670a54db21a0ad5c56f")
                .unwrap();

        let signer_2_public =
            Point::from_hex("024bae102b6f7c31045ca52e110c5155aef701469060c965dacccc41476c62c4df")
                .unwrap();

        let signer_3_secret =
            Scalar::from_hex("21df97475725ba6193450e274e5142a8c37bbbc2ae8ec5b4796b30592bccdb87")
                .unwrap();

        let signer_3_public =
            Point::from_hex("028ba1d6c14ee9ad41a0556d0033b493b36247a2a483ee975e624ff701cdec2da8")
                .unwrap();

        let mut public_list = vec![signer_1_public, signer_2_public, signer_3_public];
        public_list.sort();

        // #5 Initialize LP directory.
        let lp_dir = match LPDirectory::new(Chain::Signet) {
            Some(dir) => dir,
            None => {
                return Err("Error initializing LP directory.".into());
            }
        };

        let manager_ = DKGManager::new(&lp_dir).unwrap();
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
            let dir = manager.directory_by_height(setup_no).unwrap();
            let _dir = dir.lock().await;
            (*_dir).clone()
        };

        // populate directory with 10 DKG sessions: the first for the group key and the remaining 9 for the group nonce.
        for _ in 0..10 {
            let mut dkg_session = dkg_directory.new_session_to_fill().unwrap();

            // Communicate and fill.

            let s1_package = {
                let package = DKGPackage::new(signer_1_secret, &public_list).unwrap();
                Authenticable::new(package, signer_1_secret.serialize()).unwrap()
            };
            let s2_package = {
                let package = DKGPackage::new(signer_2_secret, &public_list).unwrap();
                Authenticable::new(package, signer_2_secret.serialize()).unwrap()
            };
            let s3_package = {
                let package = DKGPackage::new(signer_3_secret, &public_list).unwrap();
                Authenticable::new(package, signer_3_secret.serialize()).unwrap()
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

            let mut signing_session = dkg_directory
                .pick_signing_session(message, None, true)
                .unwrap();

            let s1_partial_sig = signing_session
                .partial_sign(signer_1_secret.serialize())
                .unwrap();
            let s2_partial_sig = signing_session
                .partial_sign(signer_2_secret.serialize())
                .unwrap();
            let s3_partial_sig = signing_session
                .partial_sign(signer_3_secret.serialize())
                .unwrap();

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

        println!("ara 0");

        // Musig signer #1
        let musig_signer_1_secret =
            Scalar::from_hex("77012837ab8d458df376ff778ef30575f2bb955bafe4051319aa0e7bf051a722")
                .unwrap();
        let musig_signer_1_public =
            Point::from_hex("02a04a08b82ea11ec8102168e3049b82270c19750fdbc2dc09e3c70af75020ab12")
                .unwrap();

        let musig_signer_1_hiding_secret_nonce =
            Scalar::from_hex("2bf1ce5c2e57b5080e3d67e77b0a015919dc55fd44e30d65f7817d8afd6995c6")
                .unwrap();
        let musig_signer_1_hiding_public_nonce =
            Point::from_hex("02b8e828457c6ab70e3f8a9cfdd5c21172fa69fccfeb74ec5e7a784f272ed094d6")
                .unwrap();

        let musig_signer_1_binding_secret_nonce =
            Scalar::from_hex("320b845c8ee898fd8f53ae2eacab49f0d244e9ac0ab94f5cfb2098ec654505c2")
                .unwrap();
        let musig_signer_1_binding_public_nonce =
            Point::from_hex("027923dc97eec709f6dfc56cac3eb59b5fbd47a89554202c79d9588a6dd37814ef")
                .unwrap();

        // Musig signer #2
        let musig_signer_2_secret =
            Scalar::from_hex("059c96997ffd9a48caf8c5e83267cc72d692abeb4b4d93bb4c98b8e2ff75e75e")
                .unwrap();
        let musig_signer_2_public =
            Point::from_hex("020d1ed23d3a2b909fd928e7f46d41d5878746aeac587deae1049d5e9fc72e2583")
                .unwrap();

        let musig_signer_2_hiding_secret_nonce =
            Scalar::from_hex("e0f530415cecfdae3105218d5c153c14e20a5faa2e24eb5ae11051015a48f77b")
                .unwrap();
        let musig_signer_2_hiding_public_nonce =
            Point::from_hex("023206ce056a69d097b3bf511ee15689babb11586518f78e29b33b48986c78e1c1")
                .unwrap();

        let musig_signer_2_binding_secret_nonce =
            Scalar::from_hex("e87330abaeb33e6c7f757aae7267c5e666c7d2d38ca09e27db6371aa13b2c54b")
                .unwrap();
        let musig_signer_2_binding_public_nonce =
            Point::from_hex("02571a8ceb8799f3a72685dd91d620336b2c0fe29d6e026de68257f67b539f15f0")
                .unwrap();

        // Musig signer #3
        let musig_signer_3_secret =
            Scalar::from_hex("d45a29fc831431cd086a5c44b9ec74a3121a266655cc26e6e6074cef9184519e")
                .unwrap();
        let musig_signer_3_public =
            Point::from_hex("0265e886012bd2afc676110adf8a3ad5cd39b7c210dc0abee5ee5d43f48bb73d82")
                .unwrap();

        let musig_signer_3_hiding_secret_nonce =
            Scalar::from_hex("d339d0fd55e80a656676a2b9798367762f82e36fd5c71dd8bbcde9b5c3fb923d")
                .unwrap();
        let musig_signer_3_hiding_public_nonce =
            Point::from_hex("03b8c8f30aef38216eed131e2aa37a060e18945e3372d3b85d7c548f02879fa1f6")
                .unwrap();

        let musig_signer_3_binding_secret_nonce =
            Scalar::from_hex("0e95284da81c53594cef642da41eb8d81637f4f1e854cef7d578ffa2ba32b9e6")
                .unwrap();
        let musig_signer_3_binding_public_nonce =
            Point::from_hex("03792a087c86671ebd5e52e3c64cd992a7f4f43bead3aa8177fe3324af49e51a51")
                .unwrap();

        let remote_keys = vec![
            musig_signer_1_public,
            musig_signer_2_public,
            musig_signer_3_public,
        ];

        let message = [0xffu8; 32];

        let mut noist_signing_session = dkg_directory
            .pick_signing_session(message, None, true)
            .unwrap();

        let _nonce_index = noist_signing_session.nonce_height();

        let operator_key = noist_signing_session.group_key();
        let operator_hiding_nonce = noist_signing_session.hiding_group_nonce();
        let operator_binding_nonce = noist_signing_session.post_binding_group_nonce();

        let projector = Projector::new(&remote_keys, operator_key, ProjectorTag::VTXOProjector);

        let key_agg_ctx = projector.key_agg_ctx().expect("leyn");

        let agg_key = key_agg_ctx.agg_key();

        let mut musig_ctx = MusigSessionCtx::new(&key_agg_ctx, message).unwrap();

        // Insert operator nonces:
        assert!(musig_ctx.insert_nonce(
            operator_key,
            operator_hiding_nonce,
            operator_binding_nonce
        ));

        // Insert remote nonces:
        {
            assert!(musig_ctx.insert_nonce(
                musig_signer_1_public,
                musig_signer_1_hiding_public_nonce,
                musig_signer_1_binding_public_nonce
            ));

            assert!(musig_ctx.insert_nonce(
                musig_signer_2_public,
                musig_signer_2_hiding_public_nonce,
                musig_signer_2_binding_public_nonce
            ));

            assert!(musig_ctx.insert_nonce(
                musig_signer_3_public,
                musig_signer_3_hiding_public_nonce,
                musig_signer_3_binding_public_nonce
            ));
        }

        // Musig ctx should be ready.
        assert!(musig_ctx.ready());

        assert!(noist_signing_session.set_musig_ctx(&musig_ctx));

        // Await for partial signatures ..

        // Insert operator partial signatures:
        {
            // Singatory #1
            let signatory_1_partial_sig = noist_signing_session
                .partial_sign(signer_1_secret.serialize())
                .unwrap();

            assert!(
                noist_signing_session.insert_partial_sig(signer_1_public, signatory_1_partial_sig)
            );

            // Singatory #2
            let signatory_2_partial_sig = noist_signing_session
                .partial_sign(signer_2_secret.serialize())
                .unwrap();

            assert!(
                noist_signing_session.insert_partial_sig(signer_2_public, signatory_2_partial_sig)
            );
        }

        let operator_musig_partial_sig = noist_signing_session.aggregated_sig().unwrap();
        // Insert operator musig partial sig.
        assert!(musig_ctx.insert_partial_sig(operator_key, operator_musig_partial_sig));

        // Insert remote partial signatures:
        {
            // Musig remote signer #1
            let musig_signer_1_partial_sig = musig_ctx
                .partial_sign(
                    musig_signer_1_secret,
                    musig_signer_1_hiding_secret_nonce,
                    musig_signer_1_binding_secret_nonce,
                )
                .unwrap();

            assert!(musig_ctx.insert_partial_sig(musig_signer_1_public, musig_signer_1_partial_sig));

            // Musig remote signer #2
            let musig_signer_2_partial_sig = musig_ctx
                .partial_sign(
                    musig_signer_2_secret,
                    musig_signer_2_hiding_secret_nonce,
                    musig_signer_2_binding_secret_nonce,
                )
                .unwrap();

            assert!(musig_ctx.insert_partial_sig(musig_signer_2_public, musig_signer_2_partial_sig));

            // Musig remote signer #3
            let musig_signer_3_partial_sig = musig_ctx
                .partial_sign(
                    musig_signer_3_secret,
                    musig_signer_3_hiding_secret_nonce,
                    musig_signer_3_binding_secret_nonce,
                )
                .unwrap();

            assert!(musig_ctx.insert_partial_sig(musig_signer_3_public, musig_signer_3_partial_sig));
        }

        // Full musig aggregate signature:
        let musig_agg_sig = musig_ctx.full_agg_sig().unwrap();

        assert!(schnorr::verify(
            agg_key.serialize_xonly(),
            message,
            musig_agg_sig,
            schnorr::SigningMode::BIP340
        ));

        Ok(())
    }
}
