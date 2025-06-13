use super::key::BLSSecretKey;
use crate::inscriptive::baked;
use bls_on_arkworks as bls;

/// Sign a message with a BLS secret key.
///
/// # Arguments
///
/// * `secret_key` - The BLS secret key.
/// * `message` - The message to sign.
///
pub fn bls_sign(secret_key: BLSSecretKey, message: [u8; 32]) -> [u8; 96] {
    // Get the message tag.
    let message_tag = format!("{}/{}", baked::PROJECT_TAG, "bls/message")
        .as_bytes()
        .to_vec();

    // Sign the message.
    let signature = bls::sign(secret_key, &message.to_vec(), &message_tag).unwrap();

    // Convert signature to a 96-byte array.
    signature
        .try_into()
        .expect("Unexpected BLS signature conversion error.")
}
