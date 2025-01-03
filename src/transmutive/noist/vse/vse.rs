use crate::{
    hash::{Hash, HashTag},
    into::{IntoPoint, IntoScalar},
    schnorr::{Bytes32, LiftScalar},
};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};

pub fn encrypting_key_secret(self_secret: [u8; 32], to_public: [u8; 32]) -> Option<[u8; 32]> {
    let self_secret = self_secret.into_scalar().ok()?;
    let to_public = to_public.into_point().ok()?;

    let shared_secret_point = self_secret * to_public;
    let shared_secret_point_bytes = shared_secret_point.serialize_uncompressed();
    let shared_secret_point_hash = (&shared_secret_point_bytes).hash(Some(HashTag::SharedSecret));
    let shared_secret = match MaybeScalar::reduce_from(&shared_secret_point_hash) {
        MaybeScalar::Valid(scalar) => scalar.lift(),
        MaybeScalar::Zero => Scalar::reduce_from(&shared_secret_point_hash).lift(),
    };

    Some(shared_secret.serialize())
}

pub fn encrypting_key_public(self_secret: [u8; 32], to_public: [u8; 32]) -> Option<[u8; 32]> {
    let encrypting_key_secret = encrypting_key_secret(self_secret, to_public)?;
    encrypting_key_secret.secret_to_public()
}

pub fn encrypt(secret_to_encrypt: [u8; 32], encrypting_key_secret: [u8; 32]) -> Option<[u8; 32]> {
    let secret_to_encrypt = secret_to_encrypt.into_scalar().ok()?;
    let encrypting_key_secret = encrypting_key_secret.into_scalar().ok()?;

    match secret_to_encrypt + encrypting_key_secret {
        MaybeScalar::Valid(scalar) => Some(scalar.serialize()),
        MaybeScalar::Zero => None,
    }
}

pub fn decrypt(secret_to_decrypt: [u8; 32], encrypting_key_secret: [u8; 32]) -> Option<[u8; 32]> {
    let secret_to_decrypt = secret_to_decrypt.into_scalar().ok()?;
    let encrypting_key_secret = encrypting_key_secret.into_scalar().ok()?;

    match secret_to_decrypt - encrypting_key_secret {
        MaybeScalar::Valid(scalar) => Some(scalar.serialize()),
        MaybeScalar::Zero => None,
    }
}

pub fn verify(
    combined_scalar: [u8; 32],
    public_share_point: [u8; 33], // comperessed
    vse_public_key: [u8; 32],     // xonly
) -> bool {
    let combined_scalar = match Scalar::from_slice(&combined_scalar) {
        Ok(scalar) => scalar,
        Err(_) => return false,
    };

    let public_share_point = match Point::from_slice(&public_share_point) {
        Ok(point) => point,
        Err(_) => return false,
    };

    let vse_public_key = match Point::from_slice(&vse_public_key) {
        Ok(point) => point,
        Err(_) => return false,
    };
    let combined_point = combined_scalar.base_point_mul();

    combined_point
        == match public_share_point + vse_public_key {
            MaybePoint::Valid(point) => point,
            MaybePoint::Infinity => return false,
        }
}
