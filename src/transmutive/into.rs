use crate::schnorr::Bytes32;
use secp::{MaybePoint, MaybeScalar, Point, Scalar};

pub enum ParseError {
    ParseError32,
    ParseError33,
    ParseError64,
    ParseError65,
}

#[derive(Debug)]
pub enum SecpError {
    InvalidSignature,
    InvalidScalar,
    InvalidPoint,
    SignatureParseError,
}

pub trait IntoByteArray {
    fn into_byte_array_32(&self) -> Result<[u8; 32], ParseError>;
    fn into_byte_array_33(&self) -> Result<[u8; 33], ParseError>;
    fn into_byte_array_64(&self) -> Result<[u8; 64], ParseError>;
    fn into_byte_array_65(&self) -> Result<[u8; 65], ParseError>;
}

impl IntoByteArray for Vec<u8> {
    fn into_byte_array_32(&self) -> Result<[u8; 32], ParseError> {
        let mut vec = Vec::<u8>::with_capacity(32);
        vec.extend(self);
        let bytes_32: [u8; 32] = vec.try_into().map_err(|_| ParseError::ParseError32)?;

        Ok(bytes_32)
    }

    fn into_byte_array_33(&self) -> Result<[u8; 33], ParseError> {
        let mut vec = Vec::<u8>::with_capacity(33);
        vec.extend(self);
        let bytes_33: [u8; 33] = vec.try_into().map_err(|_| ParseError::ParseError33)?;

        Ok(bytes_33)
    }

    fn into_byte_array_64(&self) -> Result<[u8; 64], ParseError> {
        let mut vec = Vec::<u8>::with_capacity(64);
        vec.extend(self);
        let bytes_64: [u8; 64] = vec.try_into().map_err(|_| ParseError::ParseError64)?;

        Ok(bytes_64)
    }

    fn into_byte_array_65(&self) -> Result<[u8; 65], ParseError> {
        let mut vec = Vec::<u8>::with_capacity(65);
        vec.extend(self);
        let bytes_65: [u8; 65] = vec.try_into().map_err(|_| ParseError::ParseError65)?;

        Ok(bytes_65)
    }
}

pub trait IntoPointVec {
    fn into_point_vec(&self) -> Result<Vec<Point>, SecpError>;
}

impl IntoPointVec for Vec<[u8; 32]> {
    fn into_point_vec(&self) -> Result<Vec<Point>, SecpError> {
        let mut points = Vec::<Point>::new();
        for point_bytes in self {
            points.push(point_bytes.into_point()?);
        }
        Ok(points)
    }
}

impl IntoPointVec for Vec<[u8; 33]> {
    fn into_point_vec(&self) -> Result<Vec<Point>, SecpError> {
        let mut points = Vec::<Point>::new();
        for point_bytes in self {
            points.push(point_bytes.into_point()?);
        }
        Ok(points)
    }
}

pub trait IntoPointByteVec {
    fn into_xpoint_vec(&self) -> Result<Vec<[u8; 32]>, SecpError>;
    fn into_cpoint_vec(&self) -> Result<Vec<[u8; 33]>, SecpError>;
}

impl IntoPointByteVec for Vec<Point> {
    fn into_xpoint_vec(&self) -> Result<Vec<[u8; 32]>, SecpError> {
        let mut point_bytes = Vec::<[u8; 32]>::new();
        for point in self {
            point_bytes.push(point.serialize_xonly());
        }
        Ok(point_bytes)
    }

    fn into_cpoint_vec(&self) -> Result<Vec<[u8; 33]>, SecpError> {
        let mut point_bytes = Vec::<[u8; 33]>::new();
        for point in self {
            point_bytes.push(point.serialize());
        }
        Ok(point_bytes)
    }
}

pub trait IntoPoint {
    fn into_point(&self) -> Result<Point, SecpError>;
}

pub trait IntoScalar {
    fn into_scalar(&self) -> Result<Scalar, SecpError>;
    fn into_reduced_scalar(&self) -> Result<Scalar, SecpError>;
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
        let mut scalar_bytes = Vec::<u8>::with_capacity(32);
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

    fn into_reduced_scalar(&self) -> Result<Scalar, SecpError> {
        let scalar = match MaybeScalar::reduce_from(&self) {
            MaybeScalar::Zero => Scalar::reduce_from(&self),
            MaybeScalar::Valid(point) => point,
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

    fn into_reduced_scalar(&self) -> Result<Scalar, SecpError> {
        let bytes: [u8; 32] = match self.clone().try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(SecpError::InvalidScalar),
        };

        let scalar = match MaybeScalar::reduce_from(&bytes) {
            MaybeScalar::Zero => Scalar::reduce_from(&bytes),
            MaybeScalar::Valid(point) => point,
        };

        Ok(scalar)
    }
}

pub trait IntoSigTuple {
    fn into_sig_tuple(&self) -> Option<(Point, Scalar)>;
}

impl IntoSigTuple for [u8; 64] {
    fn into_sig_tuple(&self) -> Option<(Point, Scalar)> {
        let public_nonce: [u8; 32] = match self[..32].try_into() {
            Ok(bytes) => bytes,
            Err(_) => return None,
        };

        let public_nonce_point = match public_nonce.to_even_point() {
            Some(public_nonce_point_) => public_nonce_point_,
            None => return None,
        };

        let s_commitment: [u8; 32] = match self[32..].try_into() {
            Ok(bytes) => bytes,
            Err(_) => return None,
        };

        let s_commitment_scalar = match Scalar::from_slice(&s_commitment) {
            Ok(scalar) => scalar,
            Err(_) => return None,
        };

        Some((public_nonce_point, s_commitment_scalar))
    }
}

pub trait FromSigTuple {
    fn from_sig_tuple(&self) -> [u8; 64];
}

impl FromSigTuple for (Point, Scalar) {
    fn from_sig_tuple(&self) -> [u8; 64] {
        let mut bytes = Vec::<u8>::with_capacity(64);
        bytes.extend(self.0.serialize_xonly());
        bytes.extend(self.1.serialize());
        bytes.try_into().expect("Unexpected FromSigTuple failure.")
    }
}
