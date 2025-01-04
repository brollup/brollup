#![allow(dead_code)]

use crate::hash::Hash;
use crate::hash::HashTag;
use rand::{rngs::OsRng, RngCore};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Signs a Schnorr message.
pub fn sign(secret_key: [u8; 32], message: [u8; 32]) -> Option<[u8; 64]> {
    // Secret-public key pairs.

    let secret_key_scalar_ = secret_key.to_scalar()?;
    let secret_key_scalar = secret_key_scalar_.lift();
    let public_key_point = secret_key_scalar.base_point_mul();

    // Secret-public nonce pairs.
    let secret_nonce_scalar_ = match secret_nonce(secret_key_scalar.serialize(), message) {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => return None,
    };
    let secret_nonce_scalar = secret_nonce_scalar_.lift();
    let public_nonce_point = secret_nonce_scalar.base_point_mul();

    // Signature challenge.
    let challenge_scalar = match challenge(public_nonce_point, public_key_point, message) {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => return None,
    };

    // Signature commitment.
    let commitment_scalar = match (secret_key_scalar * challenge_scalar) + secret_nonce_scalar {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => return None,
    };

    let mut signature = Vec::<u8>::with_capacity(64);
    signature.extend(public_nonce_point.serialize_xonly());
    signature.extend(commitment_scalar.serialize());

    signature.try_into().ok()
}

/// Verifies a Schnorr message.
pub fn verify(public_key: [u8; 32], message: [u8; 32], signature: [u8; 64]) -> bool {
    let public_key_point = match public_key.to_even_point() {
        Some(public_key_point_) => public_key_point_,
        None => return false,
    };

    let public_nonce: [u8; 32] = match signature[..32].try_into() {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let public_nonce_point = match public_nonce.to_even_point() {
        Some(public_nonce_point_) => public_nonce_point_,
        None => return false,
    };

    let challenge_scalar = match challenge(public_nonce_point, public_key_point, message) {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => return false,
    };

    let s_commitment: [u8; 32] = match signature[32..].try_into() {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let s_commitment_scalar = match Scalar::from_slice(&s_commitment) {
        Ok(scalar) => scalar,
        Err(_) => return false,
    };

    let equation_point = match (public_key_point * challenge_scalar) + public_nonce_point {
        MaybePoint::Infinity => {
            return false;
        }
        MaybePoint::Valid(point) => point,
    };

    s_commitment_scalar.base_point_mul() == equation_point
}

/// Returns signature challenge.
fn challenge(public_nonce: Point, public_key: Point, message: [u8; 32]) -> MaybeScalar {
    let mut challenge_preimage = Vec::<u8>::with_capacity(160);

    challenge_preimage.extend(public_nonce.serialize_xonly());
    challenge_preimage.extend(public_key.serialize_xonly());
    challenge_preimage.extend(message);

    let challenge = challenge_preimage.hash(Some(HashTag::SignatureChallenge));

    MaybeScalar::reduce_from(&challenge)
}

/// Deterministicially generates secret nonce for signing.
fn secret_nonce(secret_key: [u8; 32], message: [u8; 32]) -> MaybeScalar {
    let mut secret_nonce_preimage = Vec::<u8>::new();

    secret_nonce_preimage.extend(secret_key);
    secret_nonce_preimage.extend(message);

    let secret_nonce = secret_nonce_preimage.hash(Some(HashTag::SecretNonce));

    MaybeScalar::reduce_from(&secret_nonce)
}

/// Generates a random secret.
pub fn generate_secret() -> [u8; 32] {
    let mut random_entropy = [0u8; 32];
    OsRng.fill_bytes(&mut random_entropy);

    let secret = random_entropy.hash(Some(HashTag::SecretKey));
    let secret_scalar = match MaybeScalar::reduce_from(&secret) {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => Scalar::reduce_from(&secret),
    };

    secret_scalar.lift().serialize()
}

pub trait Bytes32 {
    // Conversions
    fn secret_to_public(&self) -> Option<[u8; 32]>;
    fn to_scalar(&self) -> Option<Scalar>;
    fn to_even_point(&self) -> Option<Point>;

    // Validity
    fn is_valid_secret(&self) -> bool;
    fn is_valid_public(&self) -> bool;
}

impl Bytes32 for [u8; 32] {
    /// Returns [u8; 32] x-only public key from a [u8; 32] secret key.
    fn secret_to_public(&self) -> Option<[u8; 32]> {
        let secret_key_scalar = self.to_owned().to_scalar()?;
        Some(secret_key_scalar.base_point_mul().serialize_xonly())
    }

    /// Converts [u8; 32] into a scalar.
    fn to_scalar(&self) -> Option<Scalar> {
        let scalar = match Scalar::from_slice(self) {
            Ok(scalar) => scalar,
            Err(_) => return None,
        };
        Some(scalar)
    }

    /// Converts [u8; 32] into an even point.
    fn to_even_point(&self) -> Option<Point> {
        let mut point_bytes = Vec::with_capacity(33);

        point_bytes.push(0x02);
        point_bytes.extend(self);

        match MaybePoint::from_slice(&point_bytes) {
            Ok(maybe_point) => match maybe_point {
                MaybePoint::Valid(point) => Some(point),
                MaybePoint::Infinity => None,
            },
            Err(_) => None,
        }
    }

    /// Returns whether the given bytes represent a valid scalar.
    fn is_valid_secret(&self) -> bool {
        match Scalar::from_slice(self) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Returns whether the given bytes represent a valid even point.
    fn is_valid_public(&self) -> bool {
        match self.to_even_point() {
            Some(_) => true,
            None => false,
        }
    }
}

pub trait LiftScalar {
    fn lift(&self) -> Self;
}

/// Negates secret if it has odd point.
impl LiftScalar for Scalar {
    fn lift(&self) -> Self {
        let mut secret_to_lift = *self;
        let point = secret_to_lift.base_point_mul();
        secret_to_lift = secret_to_lift.negate_if(point.parity());
        secret_to_lift
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature(pub [u8; 64]);

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SignatureVisitor;

        impl<'de> Visitor<'de> for SignatureVisitor {
            type Value = Signature;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("64 bytes")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v.len() == 64 {
                    let mut array = [0u8; 64];
                    array.copy_from_slice(v);
                    Ok(Signature(array))
                } else {
                    Err(E::invalid_length(v.len(), &self))
                }
            }
        }

        deserializer.deserialize_bytes(SignatureVisitor)
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Authenticable<T> {
    object: T,
    sig: Signature,
    key: [u8; 32],
}

impl<T> Authenticable<T>
where
    T: Sighash + Clone,
{
    pub fn new(object: T, secret_key: [u8; 32]) -> Option<Self> {
        let key = secret_key.secret_to_public()?;
        let msg = object.sighash();

        let sig = Signature(sign(secret_key, msg)?);

        Some(Self { object, sig, key })
    }

    pub fn object(&self) -> T {
        self.object.clone()
    }

    pub fn msg(&self) -> Option<[u8; 32]> {
        let authash = self.object().sighash();
        Some(authash)
    }

    pub fn sig(&self) -> [u8; 64] {
        self.sig.0
    }

    pub fn key(&self) -> [u8; 32] {
        self.key
    }

    pub fn authenticate(&self) -> bool {
        let key = self.key();
        let msg = match self.msg() {
            Some(msg) => msg,
            None => return false,
        };
        let sig = self.sig();

        verify(key, msg, sig)
    }
}

pub trait Sighash {
    fn sighash(&self) -> [u8; 32];
}
