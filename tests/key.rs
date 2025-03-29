#[cfg(test)]
mod key_tests {
    use brollup::transmutive::key::{FromNostrKeyStr, ToNostrKeyStr};
    use hex;

    #[test]
    fn to_nsec() -> Result<(), String> {
        let secret_key_bytes: [u8; 32] =
            hex::decode("bceef655b5a034911f1c3718ce056531b45ef03b4c7b1f15629e867294011a7d")
                .map_err(|_| format!("Failed to parse secret key hex."))?
                .try_into()
                .map_err(|_| "Invalid key length. Expected 32 bytes.".to_string())?;

        let nsec = secret_key_bytes
            .to_nsec()
            .ok_or_else(|| "Failed to convert secret key to nsec.".to_string())?;

        let expected_nsec =
            "nsec1hnh0v4d45q6fz8cuxuvvupt9xx69aupmf3a379tzn6r899qprf7sy22mdc".to_string();

        assert_eq!(expected_nsec, nsec);

        Ok(())
    }

    #[test]
    fn from_nsec() -> Result<(), String> {
        let nsec_str = "nsec1hnh0v4d45q6fz8cuxuvvupt9xx69aupmf3a379tzn6r899qprf7sy22mdc";

        let secret_key_bytes: [u8; 32] = nsec_str
            .from_nsec()
            .ok_or_else(|| "Failed to convert nsec str to secret key.".to_string())?;

        let expected_secret_key_bytes: [u8; 32] =
            hex::decode("bceef655b5a034911f1c3718ce056531b45ef03b4c7b1f15629e867294011a7d")
                .map_err(|_| format!("Failed to parse secret key hex."))?
                .try_into()
                .map_err(|_| "Invalid key length. Expected 32 bytes.".to_string())?;

        assert_eq!(expected_secret_key_bytes, secret_key_bytes);

        Ok(())
    }

    #[test]
    fn to_npub() -> Result<(), String> {
        let public_key_bytes: [u8; 32] =
            hex::decode("cbecda1c7d37d4c0aa5466243bb4a0018c31bf06d74fa7338290dd3068db4fed")
                .map_err(|_| format!("Failed to parse public key hex."))?
                .try_into()
                .map_err(|_| "Invalid key length. Expected 32 bytes.".to_string())?;

        let npub = public_key_bytes
            .to_npub()
            .ok_or_else(|| "Failed to convert public key to npub.".to_string())?;

        let expected_npub =
            "npub1e0kd58raxl2vp2j5vcjrhd9qqxxrr0cx6a86wvuzjrwnq6xmflks9uswpf".to_string();

        assert_eq!(expected_npub, npub);

        Ok(())
    }

    #[test]
    fn from_npub() -> Result<(), String> {
        let npub_str = "npub1e0kd58raxl2vp2j5vcjrhd9qqxxrr0cx6a86wvuzjrwnq6xmflks9uswpf";

        let public_key_bytes: [u8; 32] = npub_str
            .from_npub()
            .ok_or_else(|| "Failed to convert npub str to public key.".to_string())?;

        let expected_public_key_bytes: [u8; 32] =
            hex::decode("cbecda1c7d37d4c0aa5466243bb4a0018c31bf06d74fa7338290dd3068db4fed")
                .map_err(|_| format!("Failed to parse secret key hex."))?
                .try_into()
                .map_err(|_| "Invalid key length. Expected 32 bytes.".to_string())?;

        assert_eq!(expected_public_key_bytes, public_key_bytes);

        Ok(())
    }
}
