use super::vss::commit_shares;
use crate::transmutive::into::SecpError;
use rand::RngCore;
use secp::{MaybeScalar, Point, Scalar};

pub fn gen_polynomial(
    secret: Scalar,
    num_participants: u8,
    threshold: u8,
) -> Result<(Vec<Scalar>, Vec<Point>), SecpError> {
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

    let (secret_shares, all_coefficients) = share_shard(secret, &coefficients, num_participants)?;

    let vss_commitments = commit_shares(&all_coefficients)?;

    Ok((secret_shares, vss_commitments))
}

pub fn share_shard(
    secret: Scalar,
    coefficients: &Vec<Scalar>,
    num_shares: u8,
) -> Result<(Vec<Scalar>, Vec<Scalar>), SecpError> {
    // Prepend the secret to the coefficients
    let mut all_coefficients = Vec::<Scalar>::new();
    all_coefficients.push(secret);
    all_coefficients.extend(coefficients);

    // Evaluate the polynomial for each point x=1,...,n
    let mut secret_key_shares = Vec::<Scalar>::new();

    for x_i in 1..=num_shares {
        let mut x_i_scalar_bytes = vec![0; 31];
        x_i_scalar_bytes.push(x_i);

        let x_i_scalar = match Scalar::from_slice(&x_i_scalar_bytes) {
            Ok(scalar) => scalar,
            Err(_) => return Err(SecpError::InvalidScalar),
        };

        let y_i_scalar = polynomial_evaluate(x_i_scalar, &all_coefficients)?;

        secret_key_shares.push(y_i_scalar);
    }

    Ok((secret_key_shares, all_coefficients))
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
