use crate::transmutive::hash::{Hash, HashTag};
use crate::transmutive::into::IntoScalar;
use secp::{MaybePoint, Point, Scalar};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MusigKeyAggCtx {
    keys: Vec<Point>,
    key_coefs: Vec<Scalar>,
    agg_inner_key: Point,
    tweak: Option<Scalar>,
    agg_key: Point,
}

impl MusigKeyAggCtx {
    pub fn new(keys: &Vec<Point>, tweak: Option<Scalar>) -> Option<Self> {
        let keys = sort_keys(keys);
        let key_coefs = key_coefs(&keys)?;
        let agg_inner_key = key_agg(&keys, &key_coefs)?;

        let agg_key = match tweak {
            Some(tweak) => {
                match agg_inner_key.negate_if(agg_inner_key.parity()) + tweak.base_point_mul() {
                    MaybePoint::Valid(point) => point,
                    MaybePoint::Infinity => return None,
                }
            }
            None => agg_inner_key.clone(),
        };

        let ctx = MusigKeyAggCtx {
            keys,
            key_coefs,
            agg_inner_key,
            tweak,
            agg_key,
        };

        Some(ctx)
    }

    pub fn keys(&self) -> Vec<Point> {
        self.keys.clone()
    }

    pub fn num_keys(&self) -> usize {
        self.keys.len()
    }

    pub fn agg_inner_key(&self) -> Point {
        self.agg_inner_key.clone()
    }

    pub fn tweak(&self) -> Option<Scalar> {
        self.tweak.clone()
    }

    pub fn agg_key(&self) -> Point {
        self.agg_key.clone()
    }

    pub fn key_index(&self, key: Point) -> Option<usize> {
        self.keys.iter().position(|&x| x == key)
    }

    pub fn key_coef(&self, key: Point) -> Option<Scalar> {
        let index = self.key_index(key)?;
        let key_coef = self.key_coefs.get(index)?.to_owned();
        Some(key_coef)
    }
}

fn sort_keys(keys: &Vec<Point>) -> Vec<Point> {
    let mut keys = keys.clone();
    keys.sort();
    keys
}

fn get_second_key(keys: &Vec<Point>) -> Option<Point> {
    let second_key = keys.get(1)?;

    Some(second_key.to_owned())
}

fn hash_keys(keys: &Vec<Point>) -> Option<[u8; 32]> {
    let mut preimage = Vec::<u8>::with_capacity(keys.len() * 33);

    for key in keys {
        preimage.extend(key.serialize());
    }

    let hash = preimage.hash(Some(HashTag::KeyAggList));

    Some(hash)
}

fn keyagg_coef_internal(keys: &Vec<Point>, key: Point, second_key: Point) -> Option<Scalar> {
    let coef = match key == second_key {
        true => Scalar::one(),
        false => {
            let keys_hash = hash_keys(keys)?;

            let mut preimage = Vec::<u8>::with_capacity(65);
            preimage.extend(keys_hash);
            preimage.extend(key.serialize());

            preimage
                .hash(Some(HashTag::KeyAggCoef))
                .into_reduced_scalar()
                .ok()?
        }
    };

    Some(coef)
}

fn key_coefs(keys: &Vec<Point>) -> Option<Vec<Scalar>> {
    let second_key = get_second_key(&keys)?;

    let mut coefs = Vec::<Scalar>::with_capacity(keys.len());

    for key in keys {
        let coef = keyagg_coef_internal(keys, key.to_owned(), second_key)?;
        coefs.push(coef);
    }

    Some(coefs)
}

fn key_agg(keys: &Vec<Point>, key_coefs: &Vec<Scalar>) -> Option<Point> {
    if keys.len() != key_coefs.len() {
        return None;
    };

    let mut agg_key = MaybePoint::Infinity;

    for (index, key) in keys.iter().enumerate() {
        let key_coef = key_coefs[index];
        agg_key = agg_key + (key.to_owned() * key_coef);
    }

    let agg_key = match agg_key {
        MaybePoint::Valid(point) => point,
        MaybePoint::Infinity => return None,
    };

    Some(agg_key)
}
