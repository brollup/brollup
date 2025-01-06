use rand::RngCore;
use secp::{MaybeScalar, Point, Scalar};

use crate::into::SecpError;

use super::{lagrance::interpolating_value, vss::vss_commit};

pub fn secret_share_gen(
    secret_key: Scalar,
    num_participants: u8,
    threshold: u8,
) -> Result<(Vec<(Scalar, Scalar)>, Vec<Point>), SecpError> {
    // Generate random coefficients for the polynomial.
    let mut coefficients = Vec::<Scalar>::new();

    for _ in 0..threshold - 1 {
        let mut rng = rand::thread_rng();
        let mut coeff_bytes: Vec<u8> = vec![0; 32];

        match rng.try_fill_bytes(&mut coeff_bytes[..]) {
            Ok(_) => (),
            Err(_) => return Err(SecpError::InvalidScalar),
        };

        let coeff = match Scalar::from_slice(&coeff_bytes) {
            Ok(scalar) => scalar,
            Err(_) => return Err(SecpError::InvalidScalar),
        };

        coefficients.push(coeff);
    }

    let (participant_private_keys, coefficients) =
        secret_share_shard(secret_key, &coefficients, num_participants)?;

    let vss_commitments = vss_commit(&coefficients)?;

    Ok((participant_private_keys, vss_commitments))
}

pub fn secret_share_shard(
    s: Scalar,
    coefficients: &Vec<Scalar>,
    num_shares: u8,
) -> Result<(Vec<(Scalar, Scalar)>, Vec<Scalar>), SecpError> {
    // Prepend the secret to the coefficients
    let mut coefficients_full = Vec::<Scalar>::new();
    coefficients_full.push(s);
    coefficients_full.extend(coefficients);

    // Evaluate the polynomial for each point x=1,...,n
    let mut secret_key_shares = Vec::<(Scalar, Scalar)>::new();

    for x_i in 1..=num_shares {
        let mut x_i_scalar_bytes = vec![0; 31];
        x_i_scalar_bytes.push(x_i);

        let x_i_scalar = match Scalar::from_slice(&x_i_scalar_bytes) {
            Ok(scalar) => scalar,
            Err(_) => return Err(SecpError::InvalidScalar),
        };

        let y_i_scalar = polynomial_evaluate(x_i_scalar, &coefficients_full)?;

        secret_key_shares.push((x_i_scalar, y_i_scalar));
    }

    Ok((secret_key_shares, coefficients_full))
}

pub fn secret_share_combine(
    shares: &Vec<(Scalar, Scalar)>,
    threshold: usize,
) -> Result<Scalar, SecpError> {
    if shares.len() < threshold {
        return Err(SecpError::InvalidScalar);
    }

    let s = polynomial_interpolate_constant(shares)?;

    Ok(s)
}

fn polynomial_evaluate(x: Scalar, coeffs: &Vec<Scalar>) -> Result<Scalar, SecpError> {
    let mut value = MaybeScalar::Zero;

    let mut reversed_coeffs = coeffs.clone();
    reversed_coeffs.reverse();

    for coeff in reversed_coeffs {
        value = value * x;
        value = value + coeff;
    }

    Ok(match value {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => return Err(SecpError::InvalidScalar),
    })
}

fn polynomial_interpolate_constant(points: &Vec<(Scalar, Scalar)>) -> Result<Scalar, SecpError> {
    let mut x_coords = Vec::<Scalar>::new();

    for point in points {
        x_coords.push(point.0);
    }

    let mut f_zero: MaybeScalar = MaybeScalar::Zero;

    for point in points {
        let delta = point.1 * interpolating_value(&x_coords, point.0)?;
        f_zero += delta;
    }

    Ok(match f_zero {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => return Err(SecpError::InvalidScalar),
    })
}
