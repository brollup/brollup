use rand::RngCore;
use secp::errors::ZeroScalarError;
use secp::{MaybePoint, MaybeScalar, Point, Scalar};

fn main() {

    let hex = "781650e9b6e646b581cff8ddb57017177d832a7f3d8086aa32117c1a91b8b5cf";
    let secret = hex.parse::<Scalar>().unwrap();

    let (mut secrets, group_key, vss_commitments) = trusted_dealer_keygen(secret, 5, 3).unwrap();

    println!(
        "group key is : {}",
        hex::encode(group_key.serialize().to_vec())
    );

    for (index, secret) in secrets.iter().enumerate() {
        println!(
            "secret share {} is : {}, {}",
            index,
            hex::encode(secret.0.serialize().to_vec()),
            hex::encode(secret.1.serialize().to_vec())
        );

        println!("vss verify: {}", vss_verify((secret.0, secret.1), &vss_commitments));
    }

    // now lets combine babe

    let x1_bytes = "0000000000000000000000000000000000000000000000000000000000000001";
    let x1: Scalar = x1_bytes.parse::<Scalar>().unwrap();

    let y1_bytes = "d7b484b748281b20095bbbd967a4c3f62af09b37e422b2b0556455f04fe77202";
    let y1: Scalar = y1_bytes.parse::<Scalar>().unwrap();

    let x2_bytes = "0000000000000000000000000000000000000000000000000000000000000002";
    let x2: Scalar = x2_bytes.parse::<Scalar>().unwrap();

    let y2_bytes = "d665f2ffaf1546a693ee6b0dc24bfaa187939119ea23991c5106b34506889286";
    let y2: Scalar = y2_bytes.parse::<Scalar>().unwrap();

    let x3_bytes = "0000000000000000000000000000000000000000000000000000000000000003";
    let x3: Scalar = x3_bytes.parse::<Scalar>().unwrap();

    let y3_bytes = "c1a24473318c1e402fcf425258af63f41996e83df7aa8b6af37b6fe9cc8c929c";
    let y3: Scalar = y3_bytes.parse::<Scalar>().unwrap();

    let mut shares = Vec::<(Scalar, Scalar)>::new();

    shares.push((x1, y1));
    shares.push((x2, y2));
    shares.push((x3, y3));

    let s = secret_share_combine(&shares, 3).unwrap();

    println!("laooo {}", hex::encode(s.serialize().to_vec()));
}

#[allow(non_snake_case)]
fn vss_verify(share_i: (Scalar, Scalar), vss_commitments: &Vec<Point>) -> bool {
    let (i, sk_i) = share_i;
    let S_i = sk_i.base_point_mul();

    let mut S_i_computed = MaybePoint::Infinity;

    for j in 0..vss_commitments.len() as u8 {
        S_i_computed += vss_commitments[j as usize] * pow_scalar(i, j);
    }

    S_i == match S_i_computed {
        MaybePoint::Infinity => return false,
        MaybePoint::Valid(point) => point,
    }
}

fn pow_scalar(base: Scalar, power: u8) -> Scalar {
    let mut result = match power {
        0 => return Scalar::one(),
        _ => base,
    };

    for _ in 0..(power - 1) {
        result = result * base;
    }

    result
}

fn trusted_dealer_keygen(
    secret_key: Scalar,
    num_participants: u8,
    threshold: u8,
) -> Result<(Vec<(Scalar, Scalar)>, Point, Vec<Point>), ZeroScalarError> {
    // Generate random coefficients for the polynomial.
    let mut coefficients = Vec::<Scalar>::new();

    for i in 0..threshold - 1 {
        let mut rng = rand::thread_rng();
        let mut coeff_bytes: Vec<u8> = vec![0; 32];

        match rng.try_fill_bytes(&mut coeff_bytes[..]) {
            Ok(ok) => (),
            Err(_) => return Err(ZeroScalarError),
        };

        let coeff = match Scalar::from_slice(&coeff_bytes) {
            Ok(scalar) => scalar,
            Err(_) => return Err(ZeroScalarError),
        };

        coefficients.push(coeff);
    }

    let (participant_private_keys, coefficients) =
        secret_share_shard(secret_key, &coefficients, num_participants)?;

    let vss_commitments = vss_commit(&coefficients)?;

    Ok((
        participant_private_keys,
        vss_commitments[0],
        vss_commitments,
    ))
}

#[allow(non_snake_case)]
fn vss_commit(coeffs: &Vec<(Scalar)>) -> Result<Vec<Point>, ZeroScalarError> {
    let mut vss_commitments = Vec::<Point>::new();

    for coeff in coeffs {
        let A_i = coeff.base_point_mul();
        vss_commitments.push(A_i);
    }

    Ok(vss_commitments)
}

fn secret_share_shard(
    s: Scalar,
    coefficients: &Vec<Scalar>,
    num_shares: u8,
) -> Result<(Vec<(Scalar, Scalar)>, Vec<(Scalar)>), ZeroScalarError> {
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
            Err(_) => return Err(ZeroScalarError),
        };

        let y_i_scalar = polynomial_evaluate(x_i_scalar, &coefficients_full)?;

        secret_key_shares.push((x_i_scalar, y_i_scalar));
    }

    Ok((secret_key_shares, coefficients_full))
}

fn polynomial_evaluate(x: Scalar, coeffs: &Vec<Scalar>) -> Result<Scalar, ZeroScalarError> {
    let mut value = MaybeScalar::Zero;

    let mut reversed_coeffs = coeffs.clone();
    reversed_coeffs.reverse();

    for coeff in reversed_coeffs {
        value = value * x;
        value = value + coeff;
    }

    Ok(match value {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => return Err(ZeroScalarError),
    })
}

fn secret_share_combine(
    shares: &Vec<(Scalar, Scalar)>,
    threshold: usize,
) -> Result<Scalar, ZeroScalarError> {
    if shares.len() < threshold {
        return Err(ZeroScalarError);
    }

    let s = polynomial_interpolate_constant(shares)?;

    Ok(s)
}

fn polynomial_interpolate_constant(
    points: &Vec<(Scalar, Scalar)>,
) -> Result<Scalar, ZeroScalarError> {
    let mut x_coords = Vec::<Scalar>::new();

    for point in points {
        x_coords.push(point.0);
    }

    let mut f_zero: MaybeScalar = MaybeScalar::Zero;

    for point in points {
        let delta = point.1 * derive_interpolating_value(&x_coords, point.0)?;
        f_zero += delta;
    }

    Ok(match f_zero {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => return Err(ZeroScalarError),
    })
}

fn derive_interpolating_value(x_vec: &Vec<Scalar>, x_i: Scalar) -> Result<Scalar, ZeroScalarError> {
    let count = x_vec.iter().filter(|&&x| x == x_i).count();

    if x_vec.len() == 0 || !x_vec.contains(&x_i) || count >= 2 {
        return Err(ZeroScalarError);
    }

    let mut numerator = Scalar::one();
    let mut denominator = Scalar::one();

    let mut x_i_found = false;

    for x_j in x_vec.iter() {
        if x_i == *x_j {
            x_i_found = true;
            continue;
        }

        numerator = numerator * x_j.to_owned();

        denominator = denominator
            * match x_j.to_owned() - x_i.to_owned() {
                MaybeScalar::Valid(scalar) => scalar,
                MaybeScalar::Zero => return Err(ZeroScalarError),
            };
    }
    if !x_i_found {
        return Err(ZeroScalarError);
    }

    let result = numerator * denominator.invert();

    Ok(result)
}
