#[cfg(test)]
mod noist_tests {
    use brollup::into::IntoPointVec;
    use brollup::noist::dkg::package::DKGPackage;
    use brollup::noist::dkg::session::DKGSession;

    use brollup::noist::lagrance::{interpolating_value, lagrance_index, lagrance_index_list};
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

        let full_list = vec![signer_1_public, signer_2_public, signer_3_public];
        let full_point_list = full_list.into_point_vec().unwrap();

        // Signer 1 keymap.
        let signer_1_keymap = VSEKeyMap::new(signer_1_secret, &full_list).unwrap();
        assert!(signer_1_keymap.is_complete(&full_point_list));
        let signer_1_auth_keymap = Authenticable::new(signer_1_keymap, signer_1_secret).unwrap();
        assert!(signer_1_auth_keymap.authenticate());

        // Signer 2 keymap.
        let signer_2_keymap = VSEKeyMap::new(signer_2_secret, &full_list).unwrap();
        assert!(signer_2_keymap.is_complete(&full_point_list));
        let signer_2_auth_keymap = Authenticable::new(signer_2_keymap, signer_2_secret).unwrap();
        assert!(signer_2_auth_keymap.authenticate());

        // Signer 3 keymap.
        let signer_3_keymap = VSEKeyMap::new(signer_3_secret, &full_list).unwrap();
        assert!(signer_3_keymap.is_complete(&full_point_list));
        let signer_3_auth_keymap = Authenticable::new(signer_3_keymap, signer_3_secret).unwrap();
        assert!(signer_3_auth_keymap.authenticate());

        let mut vse_setup = VSESetup::new(&full_list, 0).unwrap();
        assert!(vse_setup.insert(signer_1_auth_keymap));
        assert!(vse_setup.insert(signer_2_auth_keymap));
        assert!(vse_setup.insert(signer_3_auth_keymap));
        assert!(vse_setup.validate());

        let package_1 = DKGPackage::new(signer_1_secret, &full_list).unwrap();
        assert!(package_1.is_complete(&full_list));
        assert!(package_1.vss_verify());
        assert!(package_1.vse_verify(&vse_setup));
        let auth_package_1 = Authenticable::new(package_1, signer_1_secret).unwrap();
        assert!(auth_package_1.authenticate());

        let package_2 = DKGPackage::new(signer_2_secret, &full_list).unwrap();
        assert!(package_2.is_complete(&full_list));
        assert!(package_2.vss_verify());
        assert!(package_2.vse_verify(&vse_setup));
        let auth_package_2 = Authenticable::new(package_2, signer_2_secret).unwrap();
        assert!(auth_package_2.authenticate());

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

        let _combined_group_point = session.group_combined_full_point(None, None).unwrap();

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

        Ok(())
    }
}
