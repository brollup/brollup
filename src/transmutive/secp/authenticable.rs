use crate::transmutive::secp::into::{IntoPoint, IntoScalar};
use crate::transmutive::secp::schnorr::{sign, verify_xonly, SchnorrSigningMode};
use secp::{Point, Scalar};
use serde::{Deserialize, Serialize};

use super::schnorr::Bytes32;

/// A trait for objects that can be authenticated.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Authenticable<T> {
    object: T,
    sig: (Point, Scalar),
    key: Point,
}

/// A trait for objects that can be authenticated.
impl<T> Authenticable<T>
where
    T: Serialize + AuthSighash + Clone,
{
    /// Create a new authenticable object.
    pub fn new(object: T, secret_key: [u8; 32]) -> Option<Self> {
        let key = secret_key.secret_to_public()?;
        let key_point = key.into_point().ok()?;

        let msg = object.auth_sighash();

        let sig = sign(secret_key, msg, SchnorrSigningMode::Cube)?;
        let nonce = &sig[..32].to_vec().into_point().ok()?;
        let s_com = &sig[32..].to_vec().into_scalar().ok()?;

        Some(Self {
            object,
            sig: (nonce.to_owned(), s_com.to_owned()),
            key: key_point,
        })
    }

    /// Serialize the authenticable object.
    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    /// Get the object.
    pub fn object(&self) -> T {
        self.object.clone()
    }

    /// Get the message.
    pub fn msg(&self) -> Option<[u8; 32]> {
        let authash = self.object().auth_sighash();
        Some(authash)
    }

    /// Get the signature.
    pub fn sig(&self) -> [u8; 64] {
        let mut sig = Vec::<u8>::with_capacity(64);
        sig.extend(self.sig.0.serialize_xonly());
        sig.extend(self.sig.1.serialize());
        sig.try_into().unwrap()
    }

    /// Get the key.
    pub fn key(&self) -> [u8; 32] {
        self.key.serialize_xonly()
    }

    /// Authenticate the object.
    pub fn authenticate(&self) -> bool {
        let key = self.key();
        let msg = match self.msg() {
            Some(msg) => msg,
            None => return false,
        };
        let sig = self.sig();

        verify_xonly(key, msg, sig, SchnorrSigningMode::Cube)
    }
}

pub trait AuthSighash {
    fn auth_sighash(&self) -> [u8; 32];
}
