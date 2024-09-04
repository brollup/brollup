use crate::encoding::conversion::IntoByteArray;

use super::schnorr::SecpError;
use secp_dep::{MaybePoint, MaybeScalar, Point, Scalar};

pub trait IntoPoint {
    fn into_point(&self) -> Result<Point, SecpError>;
}

pub trait IntoScalar {
    fn into_scalar(&self) -> Result<Scalar, SecpError>;
}

impl IntoPoint for [u8; 32] {
    fn into_point(&self) -> Result<Point, SecpError> {
        let mut point_bytes = Vec::with_capacity(33);
        point_bytes.push(0x02);
        point_bytes.extend(self);

        let point = match MaybePoint::from_slice(&point_bytes) {
            Ok(maybe_point) => match maybe_point {
                MaybePoint::Infinity => {
                    return Err(SecpError::InvalidPoint);
                }
                MaybePoint::Valid(point) => point,
            },
            Err(_) => return Err(SecpError::InvalidPoint),
        };

        Ok(point)
    }
}

impl IntoPoint for [u8; 33] {
    fn into_point(&self) -> Result<Point, SecpError> {
        let mut point_bytes = Vec::with_capacity(33);
        point_bytes.extend(self);

        let point = match MaybePoint::from_slice(&point_bytes) {
            Ok(maybe_point) => match maybe_point {
                MaybePoint::Infinity => {
                    return Err(SecpError::InvalidPoint);
                }
                MaybePoint::Valid(point) => point,
            },
            Err(_) => return Err(SecpError::InvalidPoint),
        };

        Ok(point)
    }
}

impl IntoPoint for Vec<u8> {
    fn into_point(&self) -> Result<Point, SecpError> {
        match self.len() {
            32 => {
                let mut bytes = Vec::<u8>::with_capacity(32);
                bytes.extend(self);

                let ba = bytes
                    .into_byte_array_32()
                    .map_err(|_| SecpError::InvalidPoint)?;
                return ba.into_point();
            }
            33 => {
                let mut bytes = Vec::<u8>::with_capacity(33);
                bytes.extend(self);

                let ba = bytes
                    .into_byte_array_33()
                    .map_err(|_| SecpError::InvalidPoint)?;
                return ba.into_point();
            }
            _ => return Err(SecpError::InvalidPoint),
        }
    }
}

impl IntoScalar for [u8; 32] {
    fn into_scalar(&self) -> Result<Scalar, SecpError> {
        let mut scalar_bytes = Vec::with_capacity(32);
        scalar_bytes.extend(self);

        let scalar = match MaybeScalar::from_slice(&scalar_bytes) {
            Ok(maybe_scalar) => match maybe_scalar {
                MaybeScalar::Zero => {
                    return Err(SecpError::InvalidScalar);
                }
                MaybeScalar::Valid(point) => point,
            },
            Err(_) => return Err(SecpError::InvalidScalar),
        };

        Ok(scalar)
    }
}

impl IntoScalar for Vec<u8> {
    fn into_scalar(&self) -> Result<Scalar, SecpError> {
        if self.len() != 32 {
            return Err(SecpError::InvalidPoint);
        }

        let mut bytes = Vec::<u8>::with_capacity(32);
        bytes.extend(self);

        let ba = bytes
            .into_byte_array_32()
            .map_err(|_| SecpError::InvalidPoint)?;
        ba.into_scalar()
    }
}