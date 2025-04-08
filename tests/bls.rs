#[cfg(test)]
mod bls_test {

    use bls_on_arkworks as bls;
    use brollup::transmutive::bls::bls::{
        secret_key_bytes_to_bls_secret_key, secret_key_to_bls_public_key,
    };

    #[test]
    fn test_bls_public_key() -> Result<(), String> {
        // Load known hex bytes (instead of generating a new random secret key like in the previous example)
        let secret_key_bytes = [
            0x32, 0x83, 0x88, 0xaf, 0xf0, 0xd4, 0xa5, 0xb7, 0xdc, 0x92, 0x05, 0xab, 0xd3, 0x74,
            0xe7, 0xe9, 0x8f, 0x3c, 0xd9, 0xf3, 0x41, 0x8e, 0xdb, 0x4e, 0xaf, 0xda, 0x5f, 0xb1,
            0x64, 0x73, 0xd2, 0x16,
        ];

        // Convert the secret key to a BLS public key.
        let public_key: [u8; 48] =
            secret_key_to_bls_public_key(secret_key_bytes_to_bls_secret_key(secret_key_bytes))
                .try_into()
                .unwrap();

        // Convert the expected public key to a BLS public key.
        let expected_public_key: [u8; 48] = hex::decode("9197734fff0a940bd447b35ce09e63b897c9618d8ec97eccd43b71bb3eea5af324c45af6ed36ac45783e95c79a6bfbbe")
            .unwrap()
            .try_into()
            .unwrap();

        // Check that the public key is correct.
        assert_eq!(public_key, expected_public_key);

        Ok(())
    }

    #[test]
    fn test_bls() -> Result<(), String> {
        // Load known hex bytes (instead of generating a new random secret key like in the previous example)
        let sk1 = bls::os2ip(&vec![
            0x32, 0x83, 0x88, 0xaf, 0xf0, 0xd4, 0xa5, 0xb7, 0xdc, 0x92, 0x05, 0xab, 0xd3, 0x74,
            0xe7, 0xe9, 0x8f, 0x3c, 0xd9, 0xf3, 0x41, 0x8e, 0xdb, 0x4e, 0xaf, 0xda, 0x5f, 0xb1,
            0x64, 0x73, 0xd2, 0x16,
        ]);
        let sk2 = bls::os2ip(&vec![
            0x47, 0xb8, 0x19, 0x2d, 0x77, 0xbf, 0x87, 0x1b, 0x62, 0xe8, 0x78, 0x59, 0xd6, 0x53,
            0x92, 0x27, 0x25, 0x72, 0x4a, 0x5c, 0x03, 0x1a, 0xfe, 0xab, 0xc6, 0x0b, 0xce, 0xf5,
            0xff, 0x66, 0x51, 0x38,
        ]);

        // Sign a message with the Ethereum Domain Separation Tag
        let dst = bls::DST_ETHEREUM.as_bytes().to_vec();

        println!("dst: {}", hex::encode(&dst));

        let message = "message to be signed by multiple parties"
            .as_bytes()
            .to_vec();

        println!("message: {}", hex::encode(&message));

        let first_signature = bls::sign(sk1, &message, &dst).unwrap();

        println!("first_signature: {}", hex::encode(&first_signature));

        let second_signature = bls::sign(sk2, &message, &dst).unwrap();

        println!("second_signature: {}", hex::encode(&second_signature));

        let aggregate = bls::aggregate(&vec![first_signature, second_signature]).unwrap();

        println!("aggregate: {}", hex::encode(&aggregate));

        // Derive a public key from our secret keys...
        let pk1: Vec<u8> = bls::sk_to_pk(sk1);
        let pk2: Vec<u8> = bls::sk_to_pk(sk2);

        println!("pk1: {}", hex::encode(&pk1));
        println!("pk2: {}", hex::encode(&pk2));

        // ...and verify the aggregate signature we produced.
        let verified = bls::aggregate_verify(
            vec![pk1, pk2],
            vec![message.clone(), message],
            &aggregate,
            &dst,
        );

        println!("verified: {}", verified);

        Ok(())
    }
}
