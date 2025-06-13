use crate::transmutative::hash::{Hash, HashTag};
use crate::transmutative::secp::error::SecpError;
use crate::transmutative::secp::schnorr::LiftScalar;
use secp::{MaybePoint, MaybeScalar, Point, Scalar};

pub fn encrypting_key_secret(self_secret: Scalar, to_public: Point) -> Scalar {
    let secret_point = self_secret.lift() * to_public;
    let secret_point_bytes = secret_point.serialize_uncompressed();
    let secret_point_hash = secret_point_bytes.hash(Some(HashTag::SharedSecret));
    let shared_secret = match MaybeScalar::reduce_from(&secret_point_hash) {
        MaybeScalar::Valid(scalar) => scalar.lift(),
        MaybeScalar::Zero => Scalar::reduce_from(&secret_point_hash).lift(),
    };

    shared_secret
}

pub fn encrypting_key_public(self_secret: Scalar, to_public: Point) -> Point {
    encrypting_key_secret(self_secret, to_public).base_point_mul()
}

pub fn encrypt(
    secret_to_encrypt: Scalar,
    encrypting_key_secret: Scalar,
) -> Result<Scalar, SecpError> {
    match secret_to_encrypt + encrypting_key_secret {
        MaybeScalar::Valid(scalar) => Ok(scalar),
        MaybeScalar::Zero => Err(SecpError::InvalidScalar),
    }
}

pub fn decrypt(
    encrypted_share_scalar: Scalar,
    encrypting_key_secret: Scalar,
) -> Result<Scalar, SecpError> {
    match encrypted_share_scalar - encrypting_key_secret {
        MaybeScalar::Valid(scalar) => Ok(scalar),
        MaybeScalar::Zero => Err(SecpError::InvalidScalar),
    }
}

pub fn verify(
    encrypted_share_scalar: Scalar,
    public_share_point: Point,
    encrypting_key_public: Point,
) -> bool {
    let combined_point = encrypted_share_scalar.base_point_mul();

    combined_point
        == match public_share_point + encrypting_key_public {
            MaybePoint::Valid(point) => point,
            MaybePoint::Infinity => return false,
        }
}
