#[cfg(test)]
mod secp_tests {
    use brollup::{
        encoding::conversion::IntoByteArray,
        secp::{
            into::{IntoPoint, IntoScalar},
            schnorr::{
                sign_schnorr, verify_schnorr, verify_schnorr_batch, SecpError, SignFlag
            },
            sum::{Sum, SumPoints, SumSignatures},
        },
    };

    #[test]
    fn test_sign_schnorr() -> Result<(), SecpError> {
        let message =
            hex::decode("e97f06fabc231539119048bd3c55d0aa6015ed157532e6a5e6fb15aae331791d")
                .unwrap();
        let private_key =
            hex::decode("09f5dde60c19101b671a5e3f4e6f0c0aaa92814170edf7f6bc19b5a21e358a51")
                .unwrap();
        // corresponding public key: 02dee61ab0f4cb3a993cb13c552e44f5abfbf1b377c08b0380da14de41234ea8bd

        let sig_expected = hex::decode("3cdbcc837e40a3b360f09387fd376e62b3f0c509b45a770adfd71f4006de72ab5facfd42b58fb4852a09228690349fac690b3cb261ff57f208e38c6c2a387e14").unwrap();

        let sig: [u8; 64] = sign_schnorr(
            private_key
                .into_byte_array_32()
                .map_err(|_| SecpError::SignatureParseError)?,
            message
                .into_byte_array_32()
                .map_err(|_| SecpError::SignatureParseError)?,
            SignFlag::EntrySign,
        )?;

        assert_eq!(sig.to_vec(), sig_expected);

        Ok(())
    }

    #[test]
    fn test_verify_schnorr() -> Result<(), SecpError> {
        let message =
            hex::decode("e97f06fabc231539119048bd3c55d0aa6015ed157532e6a5e6fb15aae331791d")
                .unwrap();

        let public_key =
            hex::decode("dee61ab0f4cb3a993cb13c552e44f5abfbf1b377c08b0380da14de41234ea8bd")
                .unwrap();

        // corresponding secret key: 09f5dde60c19101b671a5e3f4e6f0c0aaa92814170edf7f6bc19b5a21e358a51

        let signature = hex::decode("3cdbcc837e40a3b360f09387fd376e62b3f0c509b45a770adfd71f4006de72ab5facfd42b58fb4852a09228690349fac690b3cb261ff57f208e38c6c2a387e14").unwrap();

        verify_schnorr(
            public_key
                .into_byte_array_32()
                .map_err(|_| SecpError::SignatureParseError)?,
            message
                .into_byte_array_32()
                .map_err(|_| SecpError::SignatureParseError)?,
            signature
                .into_byte_array_64()
                .map_err(|_| SecpError::SignatureParseError)?,
            SignFlag::EntrySign,
        )
    }

    #[test]
    fn test_sum_scalars() -> Result<(), SecpError> {
        let scalar_1_bytes =
            hex::decode("d798d1fac6bd4bb1c11f50312760351013379a0ab6f0a8c0af8a506b96b2525a")
                .map_err(|_| SecpError::InvalidScalar)?;

        let scalar_1 = scalar_1_bytes.into_scalar()?;

        let scalar_2_bytes =
            hex::decode("fa22dfe1da9013b3c1145040acae9089e0c08bc1c1a0719614f4b73add6f6ef5")
                .map_err(|_| SecpError::InvalidScalar)?;

        let scalar_2 = scalar_2_bytes.into_scalar()?;

        let scalars = vec![scalar_1, scalar_2];

        let sum = scalars.sum()?;

        let expected_sum_bytes =
            hex::decode("d1bbb1dca14d5f658233a071d40ec59b394948e5c9487a1b04aca919a3eb800e")
                .map_err(|_| SecpError::InvalidScalar)?;
        let expected_sum = expected_sum_bytes.into_scalar()?;

        assert_eq!(sum, expected_sum);

        Ok(())
    }

    #[test]
    fn test_sum_points() -> Result<(), SecpError> {
        let point_1_bytes =
            hex::decode("7759eb7a3182a6e5ab4818ab2bbbb79d1aa93b16e0ef1f2b1141614a9c8402a5")
                .map_err(|_| SecpError::InvalidPoint)?;

        let point_1 = point_1_bytes.into_point()?;

        let point_2_bytes =
            hex::decode("2be00f329e405edacf4beaf1f235e1c38df8dc3a280b92573216cb8e98cc5f3c")
                .map_err(|_| SecpError::InvalidPoint)?;

        let point_2 = point_2_bytes.into_point()?;

        let points = vec![point_1, point_2];

        let sum = points.sum()?;

        let expected_sum_vec =
            hex::decode("60dadabf8a850d6f4d6ffa8ec4777bdb085e3dbb49fe6122bed3d2c3c7e0e1e3")
                .map_err(|_| SecpError::InvalidPoint)?;
        let expected_sum = expected_sum_vec.into_point()?;

        assert_eq!(sum, expected_sum);

        Ok(())
    }

    #[test]
    fn test_sum_public_keys() -> Result<(), SecpError> {
        let public_key_1_bytes =
            hex::decode("6c7216fbdd2d4f41cbb2cd16c5aad7cf142298aac5581c7d2ec2555f3e64c7c2")
                .map_err(|_| SecpError::InvalidPoint)?;

        let public_key_1 = public_key_1_bytes
            .into_byte_array_32()
            .map_err(|_| SecpError::InvalidPoint)?;

        let public_key_2_bytes =
            hex::decode("7af79ebb34707a350f375066dc7c1ba206cfc49d3963a9b4573af6ad5394402d")
                .map_err(|_| SecpError::InvalidPoint)?;

        let public_key_2 = public_key_2_bytes
            .into_byte_array_32()
            .map_err(|_| SecpError::InvalidPoint)?;

        let points = vec![public_key_1, public_key_2];

        let sum = points.sum_as_cpoints()?;

        let expected_sum_vec =
            hex::decode("03336ac1ea270659d5783b57f24338ae3a24d904e036083d3bdce1b27b97b434d1")
                .map_err(|_| SecpError::InvalidPoint)?;
        let expected_sum = expected_sum_vec
            .into_byte_array_33()
            .map_err(|_| SecpError::InvalidPoint)?;

        assert_eq!(sum, expected_sum);

        Ok(())
    }

    #[test]
    fn test_verify_schnorr_batch() -> Result<(), SecpError> {
        // 1
        let private_key_1 =
            hex::decode("d38886109880885909e45cf3cb3a13d8c7f72d454183b1724cb947180f9bcacb")
                .unwrap();
        let public_key_1 =
            hex::decode("49e92a044a315ad848951c4f135727259a6e44645813730284315a4ac7ea488a")
                .unwrap();
        let message_1 =
            hex::decode("e8ebbbaf6c4a1b1860b175cf6a8df2f9ab35897f3063aa9302e458d68c659719")
                .unwrap();

        // challange 52179be064184d2cdc410173e8406d05b0d4c0412875af8a02326af06ae5fb75
        let signature_1 = sign_schnorr(
            private_key_1
                .into_byte_array_32()
                .map_err(|_| SecpError::InvalidScalar)?,
            message_1
                .into_byte_array_32()
                .map_err(|_| SecpError::InvalidScalar)?,
            SignFlag::EntrySign,
        )?;

        // 2
        let private_key_2 =
            hex::decode("4806c330785dece02c5cdbc3484b2f42aab4b0edff45c015f3f8d5c511f4afc5")
                .unwrap();
        let public_key_2 =
            hex::decode("0d93be6ad9a4b07c5d77c8d0224ea4f6162a896ffe6a439f2a33308d20ef605f")
                .unwrap();
        let message_2 =
            hex::decode("c6fbb265fa443c72a63a3571efabbd4551765d54e223a4d5e16dc95ffca67863")
                .unwrap();

        // challenge cb39065f0eb03d8f8a0a11dd43c8515f7ff79198d5654742c830be6f1b3e3af9
        let signature_2 = sign_schnorr(
            private_key_2
                .into_byte_array_32()
                .map_err(|_| SecpError::InvalidScalar)?,
            message_2
                .into_byte_array_32()
                .map_err(|_| SecpError::InvalidScalar)?,
            SignFlag::EntrySign,
        )?;

        let signatures = vec![signature_1, signature_2];

        let signature_sum = signatures.sum()?;

        let pubkeys = vec![
            public_key_1
                .into_byte_array_32()
                .map_err(|_| SecpError::InvalidScalar)?,
            public_key_2
                .into_byte_array_32()
                .map_err(|_| SecpError::InvalidScalar)?,
        ];

        let messages = vec![
            message_1
                .into_byte_array_32()
                .map_err(|_| SecpError::InvalidScalar)?,
            message_2
                .into_byte_array_32()
                .map_err(|_| SecpError::InvalidScalar)?,
        ];

        verify_schnorr_batch(signature_sum, pubkeys, messages, SignFlag::EntrySign)
    }
}