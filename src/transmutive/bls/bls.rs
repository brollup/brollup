use crate::transmutive::hash::{Hash, HashTag};
use bls_on_arkworks::{
    self as bls,
    types::{PublicKey, SecretKey},
};

/// The type of a BLS public key.
pub type BLSPublicKey = PublicKey;

/// The type of a BLS secret key.
pub type BLSSecretKey = SecretKey;

pub fn secret_key_bytes_to_bls_secret_key(secret_key_bytes: [u8; 32]) -> BLSSecretKey {
    let tagged_secret_key = secret_key_bytes.hash(Some(HashTag::BLSSecretKey));
    bls::os2ip(&tagged_secret_key.to_vec())
}

/// Converts a 32-byte secret key to a BLS public key.
///
/// # Arguments
///
/// * `secret_key` - A 32-byte secret key.
///
pub fn secret_key_to_bls_public_key(secret_key: BLSSecretKey) -> BLSPublicKey {
    // Convert the BLS secret key to a BLS public key.
    let bls_public_key: BLSPublicKey = bls::sk_to_pk(secret_key)
        .try_into()
        .expect("Unexpected BLS public key conversion error.");

    // Return the BLS public key.
    bls_public_key
}
