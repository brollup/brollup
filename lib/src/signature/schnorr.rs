use crate::hash::{tagged_hash, HashTag};
use ::secp256k1::Secp256k1;
use secp::{MaybePoint, MaybeScalar, Point, Scalar};

use super::into::{IntoPoint, IntoScalar};

#[derive(Clone, Copy)]
pub enum SignFlag {
    BIP340Sign,
    EntrySign,
    ProtocolMessageSign,
    CustomMessageSign,
}

#[derive(Debug)]
pub enum SecpError {
    InvalidSignature,
    InvalidScalar,
    InvalidPoint,
    SignatureParseError,
}

pub fn compute_challenge(
    public_nonce: Option<Point>,
    public_key: Option<Point>,
    message_bytes: [u8; 32],
    flag: SignFlag,
) -> Result<[u8; 32], SecpError> {
    match flag {
        SignFlag::BIP340Sign => {
            // Follow BIP-340. Challenge e bytes is = H(R||P||m).

            let public_nonce = match public_nonce {
                None => return Err(SecpError::InvalidPoint),
                Some(point) => point,
            };

            let public_key = match public_key {
                None => return Err(SecpError::InvalidPoint),
                Some(point) => point,
            };

            let mut challenge_preimage = Vec::<u8>::with_capacity(96);
            challenge_preimage.extend(public_nonce.serialize_xonly());
            challenge_preimage.extend(public_key.serialize_xonly());
            challenge_preimage.extend(message_bytes);
            return Ok(tagged_hash(challenge_preimage, HashTag::BIP0340Challenge));
        }

        SignFlag::EntrySign => {
            // Do not follow BIP-340. Challange (e) bytes is = H(P||m).

            let public_key = match public_key {
                None => return Err(SecpError::InvalidPoint),
                Some(point) => point,
            };

            let mut challenge_preimage = Vec::<u8>::with_capacity(64);
            challenge_preimage.extend(public_key.serialize_xonly());
            challenge_preimage.extend(message_bytes);
            return Ok(tagged_hash(challenge_preimage, HashTag::EntryChallenge));
        }

        SignFlag::ProtocolMessageSign => {
            // Do not follow BIP-340. Challange (e) bytes is = H(P||m).

            let public_key = match public_key {
                None => return Err(SecpError::InvalidPoint),
                Some(point) => point,
            };

            let mut challenge_preimage = Vec::<u8>::with_capacity(64);
            challenge_preimage.extend(public_key.serialize_xonly());
            challenge_preimage.extend(message_bytes);
            return Ok(tagged_hash(
                challenge_preimage,
                HashTag::ProtocolMessageChallenge,
            ));
        }

        SignFlag::CustomMessageSign => {
            // Do not follow BIP-340. Challange (e) bytes is = H(m).
            return Ok(tagged_hash(message_bytes, HashTag::CustomMessageChallenge));
        }
    };
}

fn deterministic_nonce(secret_key: [u8; 32], message: [u8; 32]) -> [u8; 32] {
    let mut secret_nonce_preimage = Vec::<u8>::new();

    secret_nonce_preimage.extend(secret_key);
    secret_nonce_preimage.extend(message);

    tagged_hash(secret_nonce_preimage, HashTag::DeterministicNonce)
}

pub fn sign_schnorr(
    secret_key_bytes: [u8; 32],
    message_bytes: [u8; 32],
    flag: SignFlag,
) -> Result<[u8; 64], SecpError> {
    // Check if the secret key (d) is a valid scalar.
    let mut secret_key = secret_key_bytes.into_scalar()?;

    // Public key (P) is = dG.
    let public_key = secret_key.base_point_mul();

    // Negate the secret key (d) if it has odd public key.
    secret_key = secret_key.negate_if(public_key.parity());

    // Nonce generation is deterministic. Secret nonce (k) is = H(sk||m).
    let secret_nonce_bytes = deterministic_nonce(secret_key_bytes, message_bytes);

    // Check if the secret nonce (k) is a valid scalar.
    let mut secret_nonce = secret_nonce_bytes.into_scalar()?;

    // Public nonce (R) is = kG.
    let public_nonce = secret_nonce.base_point_mul();

    // Negate the secret nonce (k) if it has odd public nonce.
    secret_nonce = secret_nonce.negate_if(public_nonce.parity());

    // Compute the challenge (e) bytes depending on the signing method.
    let challenge_array: [u8; 32] =
        compute_challenge(Some(public_nonce), Some(public_key), message_bytes, flag)?;

    // Challange (e) is = int(challange_bytes) mod n.
    let challenge = challenge_array.into_scalar()?;

    // Commitment (s) is = k + ed mod n.
    let commitment = match secret_nonce + challenge * secret_key {
        MaybeScalar::Zero => return Err(SecpError::InvalidScalar),
        MaybeScalar::Valid(scalar) => scalar,
    };

    // Initialize the signature with a capacity of 64 bytes.
    let mut signature = Vec::<u8>::with_capacity(64);

    // Add public nonce (R) 32 bytes.
    signature.extend(public_nonce.serialize_xonly());

    // Add commitment (s) 32 bytes.
    signature.extend(commitment.serialize());

    // Signature is = bytes(R) || bytes((k + ed) mod n).
    signature
        .try_into()
        .map_err(|_| SecpError::SignatureParseError)
}

pub fn verify_schnorr(
    public_key_bytes: [u8; 32],
    message_bytes: [u8; 32],
    signature_bytes: [u8; 64],
    flag: SignFlag,
) -> Result<(), SecpError> {
    // Check if the public key (P) is a valid point.
    let public_key = public_key_bytes.into_point()?;

    // Parse public nonce (R) bytes.
    let public_nonce_bytes: [u8; 32] = (&signature_bytes[0..32])
        .try_into()
        .map_err(|_| SecpError::SignatureParseError)?;

    // Check if the public nonce (R) is a valid point.
    let public_nonce = public_nonce_bytes.into_point()?;

    // Compute the challenge (e) bytes depending on the signing method.
    let challange_array: [u8; 32] =
        compute_challenge(Some(public_nonce), Some(public_key), message_bytes, flag)?;

    // Challange (e) is = int(challange_bytes) mod n.
    let challange = challange_array.into_scalar()?;

    // Parse commitment (s) bytes.
    let commitment_bytes: [u8; 32] = (&signature_bytes[32..64])
        .try_into()
        .map_err(|_| SecpError::SignatureParseError)?;

    // Check if commitment (s) is a valid scalar.
    let commitment = commitment_bytes.into_scalar()?;

    // Check if the equation (R + eP) is a valid point.
    let equation = match public_nonce + challange * public_key {
        MaybePoint::Infinity => {
            return Err(SecpError::InvalidPoint);
        }
        MaybePoint::Valid(point) => point,
    };

    // Check if the equation (R + eP) equals to sG.
    match commitment.base_point_mul() == equation {
        false => return Err(SecpError::InvalidSignature),
        true => return Ok(()),
    }
}

pub fn verify_schnorr_batch(
    signature_sum: [u8; 64],
    public_keys: Vec<[u8; 32]>,
    messages: Vec<[u8; 32]>,
    flag: SignFlag,
) -> Result<(), SecpError> {
    let len = messages.len();

    if len == 0 {
        return Err(SecpError::InvalidPoint);
    }

    let mut challenges = Vec::<Scalar>::with_capacity(len);
    let mut public_key_points = Vec::<Point>::with_capacity(len);

    for index in 0..len {
        let public_key = public_keys[index].into_point()?;
        let message = messages[index];

        let challenge_bytes = compute_challenge(None, Some(public_key), message, flag)?;
        let challenge = challenge_bytes.into_scalar()?;

        challenges.push(challenge);
        public_key_points.push(public_key);
    }

    // Parse public nonce (R).
    let public_nonce_bytes: [u8; 32] = (&signature_sum)[0..32]
        .try_into()
        .map_err(|_| SecpError::InvalidPoint)?;
    let public_nonce = public_nonce_bytes.into_point()?;

    // Parse commitment (s).
    let commitment_bytes: [u8; 32] = (&signature_sum)[32..64]
        .try_into()
        .map_err(|_| SecpError::InvalidScalar)?;
    let commitment = commitment_bytes.into_scalar()?;

    // e1P1
    let mut challenge_times_pubkey_sum = challenges[0] * public_key_points[0];

    // e2P2..enPn
    for index in 1..challenges.len() {
        challenge_times_pubkey_sum =
            match challenge_times_pubkey_sum + challenges[index] * public_key_points[index] {
                MaybePoint::Infinity => return Err(SecpError::InvalidPoint),
                MaybePoint::Valid(point) => point,
            }
    }

    // The aggregate signature sum can have a public nonce that is either even or odd.
    // To avoid dealing with the parity bit, we check the signature against both possible equations.

    // Check if the equation (R + eP) is a valid point where the y-coordinate of R is even.
    let equation_even = match public_nonce + challenge_times_pubkey_sum {
        MaybePoint::Infinity => {
            return Err(SecpError::InvalidPoint);
        }
        MaybePoint::Valid(point) => point,
    };

    // Check if the equation (R + eP) is a valid point where the y-coordinate of R is odd.
    let equation_odd = match public_nonce.negate(&Secp256k1::new()) + challenge_times_pubkey_sum {
        MaybePoint::Infinity => {
            return Err(SecpError::InvalidPoint);
        }
        MaybePoint::Valid(point) => point,
    };

    // sG
    let commitment_times_generator = commitment.base_point_mul();

    // Check if either one of the equations (R + eP) equal to sG.
    match commitment_times_generator == equation_even {
        false => match commitment_times_generator == equation_odd {
            false => return Err(SecpError::InvalidSignature),
            true => return Ok(()),
        },
        true => return Ok(()),
    }
}