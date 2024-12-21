#[cfg(test)]
mod schnorr_tests {
    use brollup::schnorr;
    use hex;

    #[test]
    fn sign() -> Result<(), String> {
        let message: [u8; 32] =
            hex::decode("1dd8312636f6a0bf3d21fa2855e63072507453e93a5ced4301b364e91c9d87d6")
                .map_err(|_| format!("Failed to parse message hex."))?
                .try_into()
                .map_err(|_| "Failed to convert message hex.".to_string())?;

        let secret_key: [u8; 32] =
            hex::decode("2795044ce0f83f718bc79c5f2add1e52521978df91ce9b7f82c9097191d33602")
                .map_err(|_| format!("Failed to parse secret key hex."))?
                .try_into()
                .map_err(|_| "Failed to convert secret key hex.".to_string())?;

        schnorr::sign(secret_key, message).ok_or_else(|| "Failed to sign.".to_string())?;

        Ok(())
    }

    #[test]
    fn verify() -> Result<(), String> {
        let message: [u8; 32] =
            hex::decode("1dd8312636f6a0bf3d21fa2855e63072507453e93a5ced4301b364e91c9d87d6")
                .map_err(|_| format!("Failed to parse message hex."))?
                .try_into()
                .map_err(|_| "Failed to convert message hex.".to_string())?;

        let public_key: [u8; 32] =
            hex::decode("d0ea35e4a5d654109aef6b175672ea98099212a42d028fcf8bd4e38c137ff15a")
                .map_err(|_| format!("Failed to parse public key hex."))?
                .try_into()
                .map_err(|_| "Failed to convert public key hex.".to_string())?;

        let signature: [u8; 64] =
            hex::decode("ba760712dbf3f27536a73d1b8bbe2be251c7ec1e7c439e8fb3e13378dc6e445392755d60c522f6f1dda183b34560e875b4cda1823e8ffb6a0014a3a9cf5e38e1")
                .map_err(|_| format!("Failed to parse signature hex."))?
                .try_into()
                .map_err(|_| "Failed to convert signature hex.".to_string())?;

        schnorr::verify(public_key, message, signature)
            .then(|| ())
            .ok_or("Failed to verify signature.")?;

        Ok(())
    }
}
