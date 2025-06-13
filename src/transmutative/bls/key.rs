use crate::transmutative::hash::{Hash, HashTag};
use bls_on_arkworks::{
    self as bls,
    types::{PublicKey, SecretKey},
};

/// The type of a BLS public key.
pub type BLSPublicKey = PublicKey;

/// The type of a BLS secret key.
pub type BLSSecretKey = SecretKey;

/// Converts a 32-byte secret key to a BLS secret key.
///
/// # Arguments
///
/// * `secret_key_bytes` - A 32-byte secret key.
///
pub fn secret_key_bytes_to_bls_secret_key(secret_key_bytes: [u8; 32]) -> BLSSecretKey {
    // Hash the secret key with domain separation tag.
    // We also use another domain separation tag for the message during signing.
    let tagged_secret_key = secret_key_bytes.hash(Some(HashTag::BLSSecretKey));

    // Convert the tagged secret key to a BLS secret key.
    bls::os2ip(&tagged_secret_key.to_vec())
}

/// Converts a BLS secret key to a BLS public key.
///
/// # Arguments
///
/// * `secret_key` - A BLS secret key.
///
pub fn secret_key_to_bls_public_key(secret_key: BLSSecretKey) -> BLSPublicKey {
    // Convert the BLS secret key to a BLS public key.
    let bls_public_key: BLSPublicKey = bls::sk_to_pk(secret_key)
        .try_into()
        .expect("Unexpected BLS public key conversion error.");

    // Return the BLS public key.
    bls_public_key
}
