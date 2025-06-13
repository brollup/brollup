use crate::transmutative::secp::error::SecpError;
use secp::{MaybePoint, Point, Scalar};

#[allow(non_snake_case)]
pub fn commit_shares(coeffs: &Vec<Scalar>) -> Result<Vec<Point>, SecpError> {
    let mut vss_commitments = Vec::<Point>::new();

    for coeff in coeffs {
        let A_i = coeff.base_point_mul();
        vss_commitments.push(A_i);
    }

    Ok(vss_commitments)
}

#[allow(non_snake_case)]
pub fn verify_shares(share_i: (Scalar, Point), vss_commitments: &Vec<Point>) -> bool {
    let (i, P_i) = share_i;

    let mut P_i_computed = MaybePoint::Infinity;

    for j in 0..vss_commitments.len() as u32 {
        P_i_computed += vss_commitments[j as usize] * pow_scalar(i, j);
    }

    P_i == match P_i_computed {
        MaybePoint::Infinity => return false,
        MaybePoint::Valid(point) => point,
    }
}

pub fn pow_scalar(base: Scalar, power: u32) -> Scalar {
    let mut result = match power {
        0 => return Scalar::one(),
        _ => base,
    };

    for _ in 0..(power - 1) {
        result = result * base;
    }

    result
}
