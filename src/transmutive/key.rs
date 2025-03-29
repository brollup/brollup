use crate::transmutive::schnorr::Bytes32;
use bech32::{Bech32, Hrp};
use secp::{Point, Scalar};

#[derive(Clone, PartialEq)]
pub struct KeyHolder {
    secret_key: Scalar,
    nsec: String,
    public_key: Point,
    npub: String,
    nostr_keypair: nostr_sdk::Keys,
}

impl KeyHolder {
    pub fn new(secret_key: Scalar) -> Option<Self> {
        let mut public_key = secret_key.base_point_mul();

        let secret_key = secret_key.negate_if(public_key.parity());
        public_key = public_key.negate_if(public_key.parity());

        let nsec = match secret_key.serialize().to_nsec() {
            Some(nsec) => nsec,
            None => return None,
        };

        let npub = match public_key.serialize_xonly().to_npub() {
            Some(npub) => npub,
            None => return None,
        };

        let nostr_keypair = match nostr_sdk::Keys::parse(&nsec) {
            Ok(keypair) => keypair,
            Err(_) => return None,
        };

        Some(KeyHolder {
            secret_key,
            nsec,
            public_key,
            npub,
            nostr_keypair,
        })
    }

    pub fn secret_key(&self) -> Scalar {
        self.secret_key
    }

    pub fn nsec(&self) -> String {
        self.nsec.clone()
    }

    pub fn public_key(&self) -> Point {
        self.public_key
    }

    pub fn npub(&self) -> String {
        self.npub.clone()
    }

    pub fn nostr_key_pair(&self) -> nostr_sdk::Keys {
        self.nostr_keypair.clone()
    }
}

/// Trait for converting 32-byte keys into Bech32-encoded `nsec` or `npub` strings.
pub trait ToNostrKeyStr {
    /// Converts a 32-byte secret key into a Bech32-encoded `nsec` string.
    ///
    /// Returns `None` if the key is invalid.
    fn to_nsec(&self) -> Option<String>;

    /// Converts a 32-byte public key into a Bech32-encoded `npub` string.
    ///
    /// Returns `None` if the key is invalid.
    fn to_npub(&self) -> Option<String>;
}

/// Trait for decoding Bech32-encoded `nsec` or `npub` strings into 32-byte keys.
pub trait FromNostrKeyStr {
    /// Decodes a Bech32-encoded `nsec` string into a 32-byte secret key.
    ///
    /// Returns `None` if the string is invalid or doesn't represent a valid secret key.
    fn from_nsec(&self) -> Option<[u8; 32]>;

    /// Decodes a Bech32-encoded `npub` string into a 32-byte public key.
    ///
    /// Returns `None` if the string is invalid or doesn't represent a valid public key.
    fn from_npub(&self) -> Option<[u8; 32]>;
}

impl ToNostrKeyStr for [u8; 32] {
    fn to_nsec(&self) -> Option<String> {
        if !self.is_valid_secret() {
            return None;
        }

        let hrp = match Hrp::parse("nsec") {
            Ok(hrp) => hrp,
            Err(_) => return None,
        };

        let nsec = match bech32::encode::<Bech32>(hrp, self) {
            Ok(encoded) => encoded,
            Err(_) => return None,
        };

        Some(nsec)
    }

    fn to_npub(&self) -> Option<String> {
        if !self.is_valid_public() {
            return None;
        }

        let hrp = match Hrp::parse("npub") {
            Ok(hrp) => hrp,
            Err(_) => return None,
        };

        let npub = match bech32::encode::<Bech32>(hrp, self) {
            Ok(encoded) => encoded,
            Err(_) => return None,
        };

        Some(npub)
    }
}

impl FromNostrKeyStr for &str {
    fn from_nsec(&self) -> Option<[u8; 32]> {
        let (hrp, decoded_bytes) = match bech32::decode(self) {
            Ok(decoded) => decoded,
            Err(_) => return None,
        };

        if hrp.as_str() != "nsec" {
            return None;
        }

        if decoded_bytes.len() != 32 {
            return None;
        }

        let secret_key: [u8; 32] = decoded_bytes.try_into().ok()?;

        if !secret_key.is_valid_secret() {
            return None;
        }

        Some(secret_key)
    }

    fn from_npub(&self) -> Option<[u8; 32]> {
        let (hrp, decoded_bytes) = match bech32::decode(self) {
            Ok(decoded) => decoded,
            Err(_) => return None,
        };

        if hrp.as_str() != "npub" {
            return None;
        }

        if decoded_bytes.len() != 32 {
            return None;
        }

        let public_key: [u8; 32] = decoded_bytes.try_into().ok()?;

        if !public_key.is_valid_public() {
            return None;
        }

        Some(public_key)
    }
}
