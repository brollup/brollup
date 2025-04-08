use super::key::BLSPublicKey;
use crate::inscriptive::baked;
use bls_on_arkworks as bls;

/// Verify a BLS signature.
///
/// # Arguments
///
/// * `public_key` - The BLS public key.
/// * `message` - The message to verify.
/// * `signature` - The signature to verify.
pub fn bls_verify(public_key: &BLSPublicKey, message: [u8; 32], signature: [u8; 96]) -> bool {
    // Get the message tag.
    let message_tag = format!("{}/{}", baked::PROJECT_TAG, "bls/message")
        .as_bytes()
        .to_vec();

    // Verify the signature.
    bls::verify(
        public_key,
        &message.to_vec(),
        &signature.to_vec(),
        &message_tag,
    )
}

/// Verify a BLS aggregate signature.
///
/// # Arguments
///
/// * `public_keys` - The BLS public keys.
/// * `messages` - The messages to verify.
/// * `aggregate_signature` - The aggregate signature to verify.
pub fn bls_verify_aggregate(
    public_keys: Vec<BLSPublicKey>,
    messages: Vec<[u8; 32]>,
    aggregate_signature: [u8; 96],
) -> bool {
    // Get the message tag.
    let message_tag = format!("{}/{}", baked::PROJECT_TAG, "bls/message")
        .as_bytes()
        .to_vec();

    // Messages as vectors of octets.
    let messages = messages
        .into_iter()
        .map(|m| m.to_vec())
        .collect::<Vec<Vec<u8>>>();

    // Verify the aggregate signature.
    bls::aggregate_verify(
        public_keys,
        messages,
        &aggregate_signature.to_vec(),
        &message_tag,
    )
}
