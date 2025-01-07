#[cfg(test)]
mod noist_tests {
    use brollup::into::IntoPoint;
    use brollup::noist::dkg::package::DKGPackage;
    use brollup::noist::dkg::session::DKGSession;

    use brollup::noist::lagrance::{interpolating_value, lagrance_index, lagrance_index_list};
    use brollup::schnorr::{challenge, verify};
    use brollup::{
        noist::setup::{keymap::VSEKeyMap, setup::VSESetup},
        schnorr::Authenticable,
    };

    #[test]
    fn noist_test() -> Result<(), String> {
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

        let full_list = vec![signer_1_public, signer_2_public, signer_3_public];

        // Signer 1 keymap.
        let signer_1_keymap = VSEKeyMap::new(signer_1_secret, &full_list).unwrap();
        assert!(signer_1_keymap.is_complete(&full_list));
        let signer_1_auth_keymap = Authenticable::new(signer_1_keymap, signer_1_secret).unwrap();
        assert!(signer_1_auth_keymap.authenticate());

        // Signer 2 keymap.
        let signer_2_keymap = VSEKeyMap::new(signer_2_secret, &full_list).unwrap();
        assert!(signer_2_keymap.is_complete(&full_list));
        let signer_2_auth_keymap = Authenticable::new(signer_2_keymap, signer_2_secret).unwrap();
        assert!(signer_2_auth_keymap.authenticate());

        // Signer 3 keymap.
        let signer_3_keymap = VSEKeyMap::new(signer_3_secret, &full_list).unwrap();
        assert!(signer_3_keymap.is_complete(&full_list));
        let signer_3_auth_keymap = Authenticable::new(signer_3_keymap, signer_3_secret).unwrap();
        assert!(signer_3_auth_keymap.authenticate());

        let mut vse_setup = VSESetup::new(&full_list, 0).unwrap();
        assert!(vse_setup.insert(signer_1_auth_keymap));
        assert!(vse_setup.insert(signer_2_auth_keymap));
        assert!(vse_setup.insert(signer_3_auth_keymap));
        assert!(vse_setup.validate());

        // Signer 1 DKG package.
        let package_1 = DKGPackage::new(signer_1_secret, &full_list).unwrap();
        assert!(package_1.is_complete(&full_list));
        assert!(package_1.vss_verify());
        assert!(package_1.vse_verify(&vse_setup));
        let auth_package_1 = Authenticable::new(package_1, signer_1_secret).unwrap();
        assert!(auth_package_1.authenticate());

        // Signer 2 DKG package.
        let package_2 = DKGPackage::new(signer_2_secret, &full_list).unwrap();
        assert!(package_2.is_complete(&full_list));
        assert!(package_2.vss_verify());
        assert!(package_2.vse_verify(&vse_setup));
        let auth_package_2 = Authenticable::new(package_2, signer_2_secret).unwrap();
        assert!(auth_package_2.authenticate());

        // Signer 3 DKG package.
        let package_3 = DKGPackage::new(signer_3_secret, &full_list).unwrap();
        assert!(package_3.is_complete(&full_list));
        assert!(package_3.vss_verify());
        assert!(package_3.vse_verify(&vse_setup));
        let auth_package_3 = Authenticable::new(package_3, signer_3_secret).unwrap();
        assert!(auth_package_3.authenticate());

        let mut session = DKGSession::new(0, &full_list).unwrap();

        assert!(session.insert(&auth_package_1, &vse_setup));
        assert!(session.insert(&auth_package_2, &vse_setup));
        assert!(session.insert(&auth_package_3, &vse_setup));
        assert!(session.is_above_threshold());
        assert!(session.verify(&vse_setup));

        let combined_group_hiding_point = session.group_combined_hiding_point().unwrap();

        let _combined_group_pre_binding_point = session.group_combined_pre_binding_point().unwrap();

        let combined_group_post_binding_point = session
            .group_combined_post_binding_point(None, None)
            .unwrap();

        let combined_group_point = session.group_combined_full_point(None, None).unwrap();

        let s_1_hiding_secret = session
            .signatory_combined_hiding_secret(signer_1_secret)
            .unwrap();
        let s_1_post_binding_secret = session
            .signatory_combined_post_binding_secret(signer_1_secret, None, None)
            .unwrap();

        let s_2_hiding_secret = session
            .signatory_combined_hiding_secret(signer_2_secret)
            .unwrap();
        let s_2_post_binding_secret = session
            .signatory_combined_post_binding_secret(signer_2_secret, None, None)
            .unwrap();

        let s_3_hiding_secret = session
            .signatory_combined_hiding_secret(signer_3_secret)
            .unwrap();
        let s_3_post_binding_secret = session
            .signatory_combined_post_binding_secret(signer_3_secret, None, None)
            .unwrap();

        let s_1_hiding_point = session
            .signatory_combined_hiding_point(signer_1_public)
            .unwrap();
        let s_1_post_binding_point = session
            .signatory_combined_post_binding_point(signer_1_public, None, None)
            .unwrap();

        assert_eq!(s_1_hiding_secret.base_point_mul(), s_1_hiding_point);
        assert_eq!(
            s_1_post_binding_secret.base_point_mul(),
            s_1_post_binding_point
        );

        let s_2_hiding_point = session
            .signatory_combined_hiding_point(signer_2_public)
            .unwrap();
        let s_2_post_binding_point = session
            .signatory_combined_post_binding_point(signer_2_public, None, None)
            .unwrap();

        assert_eq!(s_2_hiding_secret.base_point_mul(), s_2_hiding_point);
        assert_eq!(
            s_2_post_binding_secret.base_point_mul(),
            s_2_post_binding_point
        );

        let s_3_hiding_point = session
            .signatory_combined_hiding_point(signer_3_public)
            .unwrap();
        let s_3_post_binding_point = session
            .signatory_combined_post_binding_point(signer_3_public, None, None)
            .unwrap();

        assert_eq!(s_3_hiding_secret.base_point_mul(), s_3_hiding_point);
        assert_eq!(
            s_3_post_binding_secret.base_point_mul(),
            s_3_post_binding_point
        );

        // Case #1 signatory 1 & 2 produced.
        let active_list = vec![signer_1_public, signer_2_public];
        let index_list = lagrance_index_list(&full_list, &active_list).unwrap();

        let s_1_index = lagrance_index(&full_list, signer_1_public).unwrap();
        let s_1_lagrance = interpolating_value(&index_list, s_1_index).unwrap();
        let s_1_hiding_secret_lagranced = s_1_hiding_secret * s_1_lagrance;
        let s_1_post_binding_secret_lagranced = s_1_post_binding_secret * s_1_lagrance;

        let s_2_index = lagrance_index(&full_list, signer_2_public).unwrap();
        let s_2_lagrance = interpolating_value(&index_list, s_2_index).unwrap();
        let s_2_hiding_secret_lagranced = s_2_hiding_secret * s_2_lagrance;
        let s_2_post_binding_secret_lagranced = s_2_post_binding_secret * s_2_lagrance;

        let s1_s2_combined_hiding_secret_lagranced =
            (s_1_hiding_secret_lagranced + s_2_hiding_secret_lagranced).unwrap();
        let s1_s2_combined_post_binding_secret_lagranced =
            (s_1_post_binding_secret_lagranced + s_2_post_binding_secret_lagranced).unwrap();

        assert_eq!(
            s1_s2_combined_hiding_secret_lagranced.base_point_mul(),
            combined_group_hiding_point
        );

        assert_eq!(
            s1_s2_combined_post_binding_secret_lagranced.base_point_mul(),
            combined_group_post_binding_point
        );

        // Case #2 signatory 1 & 3 produced.
        let active_list = vec![signer_1_public, signer_3_public];
        let index_list = lagrance_index_list(&full_list, &active_list).unwrap();

        let s_1_index = lagrance_index(&full_list, signer_1_public).unwrap();
        let s_1_lagrance = interpolating_value(&index_list, s_1_index).unwrap();
        let s_1_hiding_secret_lagranced = s_1_hiding_secret * s_1_lagrance;
        let s_1_post_binding_secret_lagranced = s_1_post_binding_secret * s_1_lagrance;

        let s_3_index = lagrance_index(&full_list, signer_3_public).unwrap();
        let s_3_lagrance = interpolating_value(&index_list, s_3_index).unwrap();
        let s_3_hiding_secret_lagranced = s_3_hiding_secret * s_3_lagrance;
        let s_3_post_binding_secret_lagranced = s_3_post_binding_secret * s_3_lagrance;

        let s1_s3_combined_hiding_secret_lagranced =
            (s_1_hiding_secret_lagranced + s_3_hiding_secret_lagranced).unwrap();
        let s1_s3_combined_post_binding_secret_lagranced =
            (s_1_post_binding_secret_lagranced + s_3_post_binding_secret_lagranced).unwrap();

        assert_eq!(
            s1_s3_combined_hiding_secret_lagranced.base_point_mul(),
            combined_group_hiding_point
        );

        assert_eq!(
            s1_s3_combined_post_binding_secret_lagranced.base_point_mul(),
            combined_group_post_binding_point
        );

        // Case #3 signatory 2 & 3 produced.
        let active_list = vec![signer_2_public, signer_3_public];
        let index_list = lagrance_index_list(&full_list, &active_list).unwrap();

        let s_2_index = lagrance_index(&full_list, signer_2_public).unwrap();
        let s_2_lagrance = interpolating_value(&index_list, s_2_index).unwrap();
        let s_2_hiding_secret_lagranced = s_2_hiding_secret * s_2_lagrance;
        let s_2_post_binding_secret_lagranced = s_2_post_binding_secret * s_2_lagrance;

        let s_3_index = lagrance_index(&full_list, signer_3_public).unwrap();
        let s_3_lagrance = interpolating_value(&index_list, s_3_index).unwrap();
        let s_3_hiding_secret_lagranced = s_3_hiding_secret * s_3_lagrance;
        let s_3_post_binding_secret_lagranced = s_3_post_binding_secret * s_3_lagrance;

        let s2_s3_combined_hiding_secret_lagranced =
            (s_2_hiding_secret_lagranced + s_3_hiding_secret_lagranced).unwrap();
        let s2_s3_combined_post_binding_secret_lagranced =
            (s_2_post_binding_secret_lagranced + s_3_post_binding_secret_lagranced).unwrap();

        assert_eq!(
            s2_s3_combined_hiding_secret_lagranced.base_point_mul(),
            combined_group_hiding_point
        );

        assert_eq!(
            s2_s3_combined_post_binding_secret_lagranced.base_point_mul(),
            combined_group_post_binding_point
        );

        // Case #4 all signatories 1, 2 & 3 produced.
        let active_list = vec![signer_1_public, signer_2_public, signer_3_public];
        let index_list = lagrance_index_list(&full_list, &active_list).unwrap();

        let s_1_index = lagrance_index(&full_list, signer_1_public).unwrap();
        let s_1_lagrance = interpolating_value(&index_list, s_1_index).unwrap();
        let s_1_hiding_secret_lagranced = s_1_hiding_secret * s_1_lagrance;
        let s_1_post_binding_secret_lagranced = s_1_post_binding_secret * s_1_lagrance;

        let s_2_index = lagrance_index(&full_list, signer_2_public).unwrap();
        let s_2_lagrance = interpolating_value(&index_list, s_2_index).unwrap();
        let s_2_hiding_secret_lagranced = s_2_hiding_secret * s_2_lagrance;
        let s_2_post_binding_secret_lagranced = s_2_post_binding_secret * s_2_lagrance;

        let s_3_index = lagrance_index(&full_list, signer_3_public).unwrap();
        let s_3_lagrance = interpolating_value(&index_list, s_3_index).unwrap();
        let s_3_hiding_secret_lagranced = s_3_hiding_secret * s_3_lagrance;
        let s_3_post_binding_secret_lagranced = s_3_post_binding_secret * s_3_lagrance;

        let s1_s2_s3_combined_hiding_secret_lagranced = (s_1_hiding_secret_lagranced
            + s_2_hiding_secret_lagranced
            + s_3_hiding_secret_lagranced)
            .unwrap();
        let s1_s2_s3_combined_post_binding_secret_lagranced = (s_1_post_binding_secret_lagranced
            + s_2_post_binding_secret_lagranced
            + s_3_post_binding_secret_lagranced)
            .unwrap();

        assert_eq!(
            s1_s2_s3_combined_hiding_secret_lagranced.base_point_mul(),
            combined_group_hiding_point
        );

        assert_eq!(
            s1_s2_s3_combined_post_binding_secret_lagranced.base_point_mul(),
            combined_group_post_binding_point
        );

        // Create a new session for the nonce:

        // Signer 1 DKG package for nonce.
        let package_1_nonce = DKGPackage::new(signer_1_secret, &full_list).unwrap();
        assert!(package_1_nonce.is_complete(&full_list));
        assert!(package_1_nonce.vss_verify());
        assert!(package_1_nonce.vse_verify(&vse_setup));
        let auth_package_1_nonce = Authenticable::new(package_1_nonce, signer_1_secret).unwrap();
        assert!(auth_package_1_nonce.authenticate());

        // Signer 2 DKG package for nonce.
        let package_2_nonce = DKGPackage::new(signer_2_secret, &full_list).unwrap();
        assert!(package_2_nonce.is_complete(&full_list));
        assert!(package_2_nonce.vss_verify());
        assert!(package_2_nonce.vse_verify(&vse_setup));
        let auth_package_2_nonce = Authenticable::new(package_2_nonce, signer_2_secret).unwrap();
        assert!(auth_package_2_nonce.authenticate());

        // Signer 3 DKG package for nonce.
        let package_3_nonce = DKGPackage::new(signer_3_secret, &full_list).unwrap();
        assert!(package_3_nonce.is_complete(&full_list));
        assert!(package_3_nonce.vss_verify());
        assert!(package_3_nonce.vse_verify(&vse_setup));
        let auth_package_3_nonce = Authenticable::new(package_3_nonce, signer_3_secret).unwrap();
        assert!(auth_package_3_nonce.authenticate());

        let mut session_nonce = DKGSession::new(1, &full_list).unwrap();

        assert!(session_nonce.insert(&auth_package_1_nonce, &vse_setup));
        assert!(session_nonce.insert(&auth_package_2_nonce, &vse_setup));
        assert!(session_nonce.insert(&auth_package_3_nonce, &vse_setup));
        assert!(session_nonce.is_above_threshold());
        assert!(session_nonce.verify(&vse_setup));

        // combined_group_hiding_point
        // combined_group_post_binding_point

        // challenge
        let message = [0xffu8; 32];

        let gk = Some(combined_group_point.serialize_xonly());
        let msg = Some(message);

        let combined_group_point_nonce = session_nonce.group_combined_full_point(gk, msg).unwrap();

        let group_nonce_parity = combined_group_point_nonce.parity();
        let group_key_parity = combined_group_point.parity();

        let challenge = challenge(
            combined_group_point_nonce,
            combined_group_point,
            message,
            brollup::schnorr::SigningMode::BIP340,
        )
        .unwrap();

        let s_1_hiding_secret_nonce = session_nonce
            .signatory_combined_hiding_secret(signer_1_secret)
            .unwrap();
        let s_1_hiding_public_nonce = session_nonce
            .signatory_combined_hiding_point(signer_1_public)
            .unwrap();
        let s_1_post_binding_secret_nonce = session_nonce
            .signatory_combined_post_binding_secret(signer_1_secret, gk, msg)
            .unwrap();
        let s_1_post_binding_public_nonce = session_nonce
            .signatory_combined_post_binding_point(signer_1_public, gk, msg)
            .unwrap();

        let s_2_hiding_secret_nonce = session_nonce
            .signatory_combined_hiding_secret(signer_2_secret)
            .unwrap();
        let s_2_hiding_public_nonce = session_nonce
            .signatory_combined_hiding_point(signer_2_public)
            .unwrap();
        let s_2_post_binding_secret_nonce = session_nonce
            .signatory_combined_post_binding_secret(signer_2_secret, gk, msg)
            .unwrap();
        let s_2_post_binding_public_nonce = session_nonce
            .signatory_combined_post_binding_point(signer_2_public, gk, msg)
            .unwrap();

        let s_3_hiding_secret_nonce = session_nonce
            .signatory_combined_hiding_secret(signer_3_secret)
            .unwrap();
        let s_3_hiding_public_nonce = session_nonce
            .signatory_combined_hiding_point(signer_3_public)
            .unwrap();
        let s_3_post_binding_secret_nonce = session_nonce
            .signatory_combined_post_binding_secret(signer_3_secret, gk, msg)
            .unwrap();
        let s_3_post_binding_public_nonce = session_nonce
            .signatory_combined_post_binding_point(signer_3_public, gk, msg)
            .unwrap();

        let combined_group_point_ = combined_group_point.negate_if(combined_group_point.parity());
        let combined_group_point_nonce_ =
            combined_group_point_nonce.negate_if(combined_group_point_nonce.parity());

        // Signer 1 partial signing:
        let s_1_hiding_secret_ = s_1_hiding_secret.negate_if(group_key_parity);
        let s_1_post_binding_secret_ = s_1_post_binding_secret.negate_if(group_key_parity);
        let s_1_hiding_public_ = s_1_hiding_point.negate_if(group_key_parity);
        let s_1_post_binding_public_ = s_1_post_binding_point.negate_if(group_key_parity);

        let s_1_hiding_secret_nonce_ = s_1_hiding_secret_nonce.negate_if(group_nonce_parity);
        let s_1_post_binding_secret_nonce_ =
            s_1_post_binding_secret_nonce.negate_if(group_nonce_parity);
        let s_1_hiding_public_nonce_ = s_1_hiding_public_nonce.negate_if(group_nonce_parity);
        let s_1_post_binding_public_nonce_ =
            s_1_post_binding_public_nonce.negate_if(group_nonce_parity);

        // (k + ed) + (k + ed)
        let s1_partial_sig_ = s_1_hiding_secret_nonce_
            + (challenge * s_1_hiding_secret_)
            + s_1_post_binding_secret_nonce_
            + (challenge * s_1_post_binding_secret_);

        let s1_partial_sig = s1_partial_sig_.unwrap();

        // Signer 1 partial signature verification:
        // (R + eP) + (R + eP)
        let equation_ = s_1_hiding_public_nonce_
            + (challenge * s_1_hiding_public_)
            + s_1_post_binding_public_nonce_
            + (challenge * s_1_post_binding_public_);

        let equation = equation_.unwrap();

        assert_eq!(s1_partial_sig.base_point_mul(), equation);

        // Signer 2 partial signing:
        let s_2_hiding_secret_ = s_2_hiding_secret.negate_if(group_key_parity);
        let s_2_post_binding_secret_ = s_2_post_binding_secret.negate_if(group_key_parity);
        let s_2_hiding_public_ = s_2_hiding_point.negate_if(group_key_parity);
        let s_2_post_binding_public_ = s_2_post_binding_point.negate_if(group_key_parity);

        let s_2_hiding_secret_nonce_ = s_2_hiding_secret_nonce.negate_if(group_nonce_parity);
        let s_2_post_binding_secret_nonce_ =
            s_2_post_binding_secret_nonce.negate_if(group_nonce_parity);
        let s_2_hiding_public_nonce_ = s_2_hiding_public_nonce.negate_if(group_nonce_parity);
        let s_2_post_binding_public_nonce_ =
            s_2_post_binding_public_nonce.negate_if(group_nonce_parity);

        // (k + ed) + (k + ed)
        let s2_partial_sig_ = s_2_hiding_secret_nonce_
            + (challenge * s_2_hiding_secret_)
            + s_2_post_binding_secret_nonce_
            + (challenge * s_2_post_binding_secret_);

        let s2_partial_sig = s2_partial_sig_.unwrap();

        // Signer 2 partial signature verification:
        // (R + eP) + (R + eP)
        let equation_ = s_2_hiding_public_nonce_
            + (challenge * s_2_hiding_public_)
            + s_2_post_binding_public_nonce_
            + (challenge * s_2_post_binding_public_);

        let equation = equation_.unwrap();

        assert_eq!(s2_partial_sig.base_point_mul(), equation);

        // Signer 3 partial signing:
        let s_3_hiding_secret_ = s_3_hiding_secret.negate_if(group_key_parity);
        let s_3_post_binding_secret_ = s_3_post_binding_secret.negate_if(group_key_parity);
        let s_3_hiding_public_ = s_3_hiding_point.negate_if(group_key_parity);
        let s_3_post_binding_public_ = s_3_post_binding_point.negate_if(group_key_parity);

        let s_3_hiding_secret_nonce_ = s_3_hiding_secret_nonce.negate_if(group_nonce_parity);
        let s_3_post_binding_secret_nonce_ =
            s_3_post_binding_secret_nonce.negate_if(group_nonce_parity);
        let s_3_hiding_public_nonce_ = s_3_hiding_public_nonce.negate_if(group_nonce_parity);
        let s_3_post_binding_public_nonce_ =
            s_3_post_binding_public_nonce.negate_if(group_nonce_parity);

        // (k + ed) + (k + ed)
        let s3_partial_sig_ = s_3_hiding_secret_nonce_
            + (challenge * s_3_hiding_secret_)
            + s_3_post_binding_secret_nonce_
            + (challenge * s_3_post_binding_secret_);

        let s3_partial_sig = s3_partial_sig_.unwrap();

        // Signer 1 partial signature verification:
        // (R + eP) + (R + eP)
        let equation_ = s_3_hiding_public_nonce_
            + (challenge * s_3_hiding_public_)
            + s_3_post_binding_public_nonce_
            + (challenge * s_3_post_binding_public_);

        let equation = equation_.unwrap();

        assert_eq!(s3_partial_sig.base_point_mul(), equation);

        // Case #1 signer 1 & 2 produced.
        let active_list = vec![signer_1_public, signer_2_public];
        let index_list = lagrance_index_list(&full_list, &active_list).unwrap();

        let s1_index = lagrance_index(&full_list, signer_1_public).unwrap();
        let s1_lagrance = interpolating_value(&index_list, s1_index).unwrap();

        let s2_index = lagrance_index(&full_list, signer_2_public).unwrap();
        let s2_lagrance = interpolating_value(&index_list, s2_index).unwrap();

        let s1_s2_agg_sig_ = (s1_partial_sig * s1_lagrance) + (s2_partial_sig * s2_lagrance);
        let s1_s2_agg_sig = s1_s2_agg_sig_.unwrap();

        let equation_ = combined_group_point_nonce_ + (challenge * combined_group_point_);
        let equation = equation_.unwrap();

        assert_eq!(s1_s2_agg_sig.base_point_mul(), equation);

        let mut s1_s2_full_sig_ = Vec::<u8>::with_capacity(64);
        s1_s2_full_sig_.extend(combined_group_point_nonce.serialize_xonly());
        s1_s2_full_sig_.extend(s1_s2_agg_sig.serialize());

        let s1_s2_full_sig: [u8; 64] = s1_s2_full_sig_.try_into().unwrap();

        assert!(verify(
            combined_group_point.serialize_xonly(),
            message,
            s1_s2_full_sig,
            brollup::schnorr::SigningMode::BIP340
        ));

        // Case #2 signer 1 & 3 produced.
        let active_list = vec![signer_1_public, signer_3_public];
        let index_list = lagrance_index_list(&full_list, &active_list).unwrap();

        let s1_index = lagrance_index(&full_list, signer_1_public).unwrap();
        let s1_lagrance = interpolating_value(&index_list, s1_index).unwrap();

        let s3_index = lagrance_index(&full_list, signer_3_public).unwrap();
        let s3_lagrance = interpolating_value(&index_list, s3_index).unwrap();

        let s1_s3_agg_sig_ = (s1_partial_sig * s1_lagrance) + (s3_partial_sig * s3_lagrance);
        let s1_s3_agg_sig = s1_s3_agg_sig_.unwrap();

        let equation_ = combined_group_point_nonce_ + (challenge * combined_group_point_);
        let equation = equation_.unwrap();

        assert_eq!(s1_s3_agg_sig.base_point_mul(), equation);

        let mut s1_s3_full_sig_ = Vec::<u8>::with_capacity(64);
        s1_s3_full_sig_.extend(combined_group_point_nonce.serialize_xonly());
        s1_s3_full_sig_.extend(s1_s3_agg_sig.serialize());

        let s1_s3_full_sig: [u8; 64] = s1_s3_full_sig_.try_into().unwrap();

        assert!(verify(
            combined_group_point.serialize_xonly(),
            message,
            s1_s3_full_sig,
            brollup::schnorr::SigningMode::BIP340
        ));

        // Case #3 signer 2 & 3 produced.
        let active_list = vec![signer_2_public, signer_3_public];
        let index_list = lagrance_index_list(&full_list, &active_list).unwrap();

        let s2_index = lagrance_index(&full_list, signer_2_public).unwrap();
        let s2_lagrance = interpolating_value(&index_list, s2_index).unwrap();

        let s3_index = lagrance_index(&full_list, signer_3_public).unwrap();
        let s3_lagrance = interpolating_value(&index_list, s3_index).unwrap();

        let s2_s3_agg_sig_ = (s2_partial_sig * s2_lagrance) + (s3_partial_sig * s3_lagrance);
        let s2_s3_agg_sig = s2_s3_agg_sig_.unwrap();

        let equation_ = combined_group_point_nonce_ + (challenge * combined_group_point_);
        let equation = equation_.unwrap();

        assert_eq!(s2_s3_agg_sig.base_point_mul(), equation);

        let mut s2_s3_full_sig_ = Vec::<u8>::with_capacity(64);
        s2_s3_full_sig_.extend(combined_group_point_nonce.serialize_xonly());
        s2_s3_full_sig_.extend(s2_s3_agg_sig.serialize());

        let s2_s3_full_sig: [u8; 64] = s2_s3_full_sig_.try_into().unwrap();

        assert!(verify(
            combined_group_point.serialize_xonly(),
            message,
            s2_s3_full_sig,
            brollup::schnorr::SigningMode::BIP340
        ));

        // Case #4 all signers 1, 2 & 3 produced.
        let active_list = vec![signer_1_public, signer_2_public, signer_3_public];
        let index_list = lagrance_index_list(&full_list, &active_list).unwrap();

        let s1_index = lagrance_index(&full_list, signer_1_public).unwrap();
        let s1_lagrance = interpolating_value(&index_list, s1_index).unwrap();

        let s2_index = lagrance_index(&full_list, signer_2_public).unwrap();
        let s2_lagrance = interpolating_value(&index_list, s2_index).unwrap();

        let s3_index = lagrance_index(&full_list, signer_3_public).unwrap();
        let s3_lagrance = interpolating_value(&index_list, s3_index).unwrap();

        let s1_s2_s3_agg_sig_ = (s1_partial_sig * s1_lagrance)
            + (s2_partial_sig * s2_lagrance)
            + (s3_partial_sig * s3_lagrance);
        let s1_s2_s3_agg_sig = s1_s2_s3_agg_sig_.unwrap();

        let equation_ = combined_group_point_nonce_ + (challenge * combined_group_point_);
        let equation = equation_.unwrap();

        assert_eq!(s1_s2_s3_agg_sig.base_point_mul(), equation);

        let mut s1_s2_s3_full_sig_ = Vec::<u8>::with_capacity(64);
        s1_s2_s3_full_sig_.extend(combined_group_point_nonce.serialize_xonly());
        s1_s2_s3_full_sig_.extend(s1_s2_s3_agg_sig.serialize());

        let s1_s2_s3_full_sig: [u8; 64] = s1_s2_s3_full_sig_.try_into().unwrap();

        assert!(verify(
            combined_group_point.serialize_xonly(),
            message,
            s1_s2_s3_full_sig,
            brollup::schnorr::SigningMode::BIP340
        ));

        Ok(())
    }
}
