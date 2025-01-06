use secp::{MaybeScalar, Scalar};

use crate::into::SecpError;

pub fn interpolating_value(x_vec: &Vec<Scalar>, x_i: Scalar) -> Result<Scalar, SecpError> {
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

pub fn lagrance_index_list(
    full_list: &Vec<[u8; 32]>,
    active_list: &Vec<[u8; 32]>,
) -> Option<Vec<Scalar>> {
    let mut active_list = active_list.clone();
    active_list.sort();

    let mut index_list = Vec::<Scalar>::new();

    for active in active_list.iter() {
        if !full_list.contains(active) {
            return None;
        };
        let active_lagrance_index = lagrance_index(&full_list, active.to_owned())?;
        index_list.push(active_lagrance_index);
    }

    Some(index_list)
}

pub fn lagrance_index(full_list: &Vec<[u8; 32]>, signatory: [u8; 32]) -> Option<Scalar> {
    let mut full_list = full_list.clone();
    full_list.sort();

    for (index, signatory_key) in full_list.iter().enumerate() {
        if signatory_key == &signatory {
            match MaybeScalar::from((index + 1) as u128) {
                MaybeScalar::Valid(scalar) => return Some(scalar),
                MaybeScalar::Zero => return None,
            }
        }
    }

    None
}
