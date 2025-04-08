#[cfg(test)]
mod musig_standalone {
    use brollup::transmutive::{
        musig::{keyagg::MusigKeyAggCtx, session::MusigSessionCtx},
        secp::schnorr::{self, SchnorrSigningMode},
    };
    use secp::{Point, Scalar};

    #[test]
    fn test_musig_standalone() -> Result<(), String> {
        let signer_1_secret_key: Scalar =
            Scalar::from_hex("1cc5906ab936b1e29db24fffe9f87b33a4c64f2d3b59aed6c3c4faeb8fcba6da")
                .unwrap();
        let signer_1_public_key: Point =
            Point::from_hex("02cb70281face51a77d51400612196032bb12422d4c07fa42997a0ab39c2431455")
                .unwrap();

        let signer_2_secret_key: Scalar =
            Scalar::from_hex("4882eef979baa5c88fd9e62c698de201f0a991af65877becf683e988f3024b0f")
                .unwrap();
        let signer_2_public_key: Point =
            Point::from_hex("0251deb9fcf4d16b0f82c75cf71e1ffb7879beb0c6bf733b0778a81b777406574f")
                .unwrap();

        let signer_3_secret_key: Scalar =
            Scalar::from_hex("2c71bfbd0389b96e292b37c2272ea846655cfb48578b06600c0ffd991f6f7e29")
                .unwrap();
        let signer_3_public_key: Point =
            Point::from_hex("029611bc66d526fa3194d0f525dce21e782dcf90cc72529ec2d5486da838d83770")
                .unwrap();

        let keys = vec![
            signer_1_public_key,
            signer_2_public_key,
            signer_3_public_key,
        ];

        let key_agg_ctx = MusigKeyAggCtx::new(&keys, None).unwrap();

        let agg_key = key_agg_ctx.agg_key();

        let agg_key_expected =
            Point::from_hex("02ea571f5841a9fae358e12d80b9073b5ab7c07aabd62051698b7ff962f13fda05")
                .unwrap();

        assert_eq!(agg_key, agg_key_expected);

        let message = [0xffu8; 32];

        let mut session_ctx = MusigSessionCtx::new(&key_agg_ctx, message).unwrap();

        // Siner 1 inserting their nonce.

        let signer_1_hiding_secret_nonce: Scalar =
            Scalar::from_hex("e2d64e2bd20d5843d03a47199f059aebdf2a9904616a01fe961ee875a7748199")
                .unwrap();
        let signer_1_hiding_public_nonce: Point =
            Point::from_hex("020f8eb9edf13c5cbca406d616d9441311906d72ea405bcb7e22b99f7e892f0d20")
                .unwrap();

        let signer_1_binding_secret_nonce: Scalar =
            Scalar::from_hex("4b978d3aac4135213f536194522f68fbb2ca4321a49d95560ae9726cd9d6a55d")
                .unwrap();
        let signer_1_binding_public_nonce: Point =
            Point::from_hex("031451a7f53decf60829622152e16f92b9fb7b72b4521e03510eba2469a742643f")
                .unwrap();

        assert!(session_ctx.insert_nonce(
            signer_1_public_key,
            signer_1_hiding_public_nonce,
            signer_1_binding_public_nonce,
        ));

        assert_eq!(session_ctx.ready(), false);

        // Siner 2 inserting their nonce.

        let signer_2_hiding_secret_nonce: Scalar =
            Scalar::from_hex("d3b9f2f01f7caa9b0fe2e932ae752f71da9f8f1a652ec895504091333b97d007")
                .unwrap();
        let signer_2_hiding_public_nonce: Point =
            Point::from_hex("024cb6badc87cfcad700eb028e1203f2cc0fd63a919d7c199a63b7891afd300e7c")
                .unwrap();

        let signer_2_binding_secret_nonce: Scalar =
            Scalar::from_hex("961a4d128a1f3cb5c41e71bc86fdc9e81050b7471f05112a6a5360a2240ff3cf")
                .unwrap();
        let signer_2_binding_public_nonce: Point =
            Point::from_hex("02f963d471e593d7574451d73a748ed06edae936f62cda9b4b62aa9cdd280c1d99")
                .unwrap();

        assert!(session_ctx.insert_nonce(
            signer_2_public_key,
            signer_2_hiding_public_nonce,
            signer_2_binding_public_nonce,
        ));

        assert_eq!(session_ctx.ready(), false);

        // Siner 3 inserting their nonce.

        let signer_3_hiding_secret_nonce: Scalar =
            Scalar::from_hex("cf2087a05db9aad43ae97aba584f8d8cb9d61fb84c39f372ea72bdd1d272ab81")
                .unwrap();
        let signer_3_hiding_public_nonce: Point =
            Point::from_hex("03e7e1a1b3ea5aa793f28b6122f28b875e5f5b01d1dc8dc886c83e5c968c980ef0")
                .unwrap();

        let signer_3_binding_secret_nonce: Scalar =
            Scalar::from_hex("4025f894ab8712c244e38af85094043e025824a0d021cd6fb9709fc9ef739e45")
                .unwrap();
        let signer_3_binding_public_nonce: Point =
            Point::from_hex("0238469201a552f6428bf11c05c64b28022a75b848c826e30449e0b4e37523e3f7")
                .unwrap();

        assert!(session_ctx.insert_nonce(
            signer_3_public_key,
            signer_3_hiding_public_nonce,
            signer_3_binding_public_nonce,
        ));

        assert_eq!(session_ctx.ready(), true);

        // Agg nonce

        let agg_nonce = session_ctx.agg_nonce().unwrap();

        let agg_nonce_expected =
            Point::from_hex("020a8a4ce4663eaee2c2c2a0426db9cc503e9eb28c349377c264f87902d726a41b")
                .unwrap();

        assert_eq!(agg_nonce, agg_nonce_expected);

        // Signer 1 partial signing:

        let signer_1_partial_sig = session_ctx
            .partial_sign(
                signer_1_secret_key,
                signer_1_hiding_secret_nonce,
                signer_1_binding_secret_nonce,
            )
            .unwrap();

        assert!(session_ctx.insert_partial_sig(signer_1_public_key, signer_1_partial_sig));

        // Signer 2 partial signing:

        let signer_2_partial_sig = session_ctx
            .partial_sign(
                signer_2_secret_key,
                signer_2_hiding_secret_nonce,
                signer_2_binding_secret_nonce,
            )
            .unwrap();

        assert!(session_ctx.insert_partial_sig(signer_2_public_key, signer_2_partial_sig));

        // Signer 3 partial signing:

        let signer_3_partial_sig = session_ctx
            .partial_sign(
                signer_3_secret_key,
                signer_3_hiding_secret_nonce,
                signer_3_binding_secret_nonce,
            )
            .unwrap();

        assert!(session_ctx.insert_partial_sig(signer_3_public_key, signer_3_partial_sig));

        let full_agg_sig = session_ctx.full_agg_sig().unwrap();

        assert!(schnorr::verify(
            agg_key.serialize_xonly(),
            message,
            full_agg_sig,
            SchnorrSigningMode::BIP340
        ));

        Ok(())
    }
}
