use super::into::IntoPoint;
use super::into::IntoScalar;
use super::schnorr::SecpError;
use secp_dep::MaybePoint;
use secp_dep::MaybeScalar;
use secp_dep::Point;
use secp_dep::Scalar;

pub trait SumType {}

impl SumType for Scalar {}
impl SumType for Point {}

pub trait Sum<T: SumType> {
    fn sum(&self) -> Result<T, SecpError>;
}

pub trait SumScalars {
    fn sum_as_scalars(&self) -> Result<[u8; 32], SecpError>;
}

pub trait SumPoints {
    fn sum_as_points(&self) -> Result<[u8; 33], SecpError>;
}

pub trait SumSignatures {
    fn sum(&self) -> Result<[u8; 65], SecpError>;
}

impl Sum<Scalar> for Vec<Scalar> {
    fn sum(&self) -> Result<Scalar, SecpError> {
        if self.len() == 0 {
            return Err(SecpError::InvalidScalar);
        }

        let mut sum = self[0];

        for scalar in self.iter().skip(1) {
            sum = match sum + *scalar {
                MaybeScalar::Zero => return Err(SecpError::InvalidScalar),
                MaybeScalar::Valid(scalar) => scalar,
            };
        }

        Ok(sum)
    }
}

impl Sum<Point> for Vec<Point> {
    fn sum(&self) -> Result<Point, SecpError> {
        if self.len() == 0 {
            return Err(SecpError::InvalidPoint);
        }

        let mut sum = self[0];

        for point in self.iter().skip(1) {
            sum = match sum + *point {
                MaybePoint::Infinity => return Err(SecpError::InvalidPoint),
                MaybePoint::Valid(point) => point,
            };
        }

        Ok(sum)
    }
}

impl SumScalars for Vec<[u8; 32]> {
    fn sum_as_scalars(&self) -> Result<[u8; 32], SecpError> {
        let mut scalars = Vec::<Scalar>::with_capacity(self.len());

        for scalar_bytes in self {
            let scalar = scalar_bytes.into_scalar()?;
            scalars.push(scalar);
        }

        let sum = scalars.sum()?;

        Ok(sum.serialize())
    }
}

impl SumPoints for Vec<[u8; 32]> {
    fn sum_as_points(&self) -> Result<[u8; 33], SecpError> {
        let mut points = Vec::<Point>::with_capacity(self.len());

        for point_bytes in self {
            let point = point_bytes.into_point()?;
            points.push(point);
        }

        let sum = points.sum()?;

        Ok(sum.serialize())
    }
}

impl SumSignatures for Vec<[u8; 64]> {
    fn sum(&self) -> Result<[u8; 65], SecpError> {
        let mut public_nonces = Vec::<[u8; 32]>::with_capacity(self.len());
        let mut commitments = Vec::<[u8; 32]>::with_capacity(self.len());

        for signature in self {
            let public_nonce: [u8; 32] = (&signature[0..32])
                .try_into()
                .map_err(|_| SecpError::InvalidPoint)?;
            public_nonces.push(public_nonce);

            let commitment: [u8; 32] = (&signature[32..64])
                .try_into()
                .map_err(|_| SecpError::InvalidScalar)?;
            commitments.push(commitment);
        }

        let public_nonces_sum = public_nonces.sum_as_points()?;
        let commitments_sum = commitments.sum_as_scalars()?;

        let mut signature = [0u8; 65];

        signature[..33].copy_from_slice(&public_nonces_sum[0..33]);
        signature[33..].copy_from_slice(&commitments_sum);

        Ok(signature)
    }
}
