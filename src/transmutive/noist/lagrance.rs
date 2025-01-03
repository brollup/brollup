use secp::{MaybeScalar, Scalar};

use crate::into::SecpError;

pub fn lagrance_interpolating_value(x_vec: &Vec<Scalar>, x_i: Scalar) -> Result<Scalar, SecpError> {
    if x_vec.len() == 0 || !x_vec.contains(&x_i) {
        return Err(SecpError::InvalidScalar);
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
                MaybeScalar::Zero => return Err(SecpError::InvalidScalar),
            };
    }
    if !x_i_found {
        return Err(SecpError::InvalidScalar);
    }

    let result = numerator * denominator.invert();

    Ok(result)
}