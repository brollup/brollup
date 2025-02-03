use crate::{
    hash::{Hash, HashTag},
    into::IntoScalar,
};

use super::keyagg::KeyAggCtx;
use secp::{MaybePoint, Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct SessionCtx {
    // Initialization
    key_agg_ctx: KeyAggCtx,
    // Set message
    message: [u8; 32],
    // Insert nonces
    nonces: HashMap<Point, (Point, Point)>, // Hiding and binding nonces.
    // Fill sigs
    partial_sigs: HashMap<Point, Scalar>,
}

impl SessionCtx {
    pub fn new(key_agg_ctx: &KeyAggCtx, message: [u8; 32]) -> Option<Self> {
        let ctx = SessionCtx {
            key_agg_ctx: key_agg_ctx.to_owned(),
            message,
            nonces: HashMap::<Point, (Point, Point)>::new(),
            partial_sigs: HashMap::<Point, Scalar>::new(),
        };

        Some(ctx)
    }

    pub fn insert_nonce(&mut self, key: Point, hiding_nonce: Point, binding_nonce: Point) -> bool {
        if let None = self.key_agg_ctx.key_index(key) {
            return false;
        }

        if let Some(_) = self.nonces.insert(key, (hiding_nonce, binding_nonce)) {
            return false;
        }

        true
    }

    pub fn nonces_ready(&self) -> bool {
        self.key_agg_ctx.num_keys() == self.nonces.len()
    }
}

fn nonce_coef(
    hiding_agg_nonce: Point,
    binding_agg_nonce: Point,
    agg_key: Point,
    message: [u8; 32],
) -> Option<Scalar> {
    let mut preimage = Vec::<u8>::with_capacity(130);

    preimage.extend(hiding_agg_nonce.serialize());
    preimage.extend(binding_agg_nonce.serialize());
    preimage.extend(agg_key.serialize_xonly());
    preimage.extend(message);

    let coef = preimage
        .hash(Some(HashTag::MusigNonceCoef))
        .into_reduced_scalar()
        .ok()?;

    Some(coef)
}

fn nonce_agg(
    nonces: &HashMap<Point, (Point, Point)>,
    agg_key: Point,
    message: [u8; 32],
) -> Option<Point> {
    let (hiding_agg_nonce, binding_agg_nonce) = pre_nonce_agg(nonces)?;
    let nonce_coef = nonce_coef(hiding_agg_nonce, binding_agg_nonce, agg_key, message)?;

    let agg_nonce = match hiding_agg_nonce + (binding_agg_nonce * nonce_coef) {
        MaybePoint::Valid(point) => point,
        MaybePoint::Infinity => return None,
    };

    Some(agg_nonce)
}

fn pre_nonce_agg(nonces: &HashMap<Point, (Point, Point)>) -> Option<(Point, Point)> {
    let mut sorted_nonces: Vec<_> = nonces.into_iter().collect();
    sorted_nonces.sort_by_key(|(key, _)| *key);

    let mut hiding_agg_nonce_ = MaybePoint::Infinity;
    let mut binding_agg_nonce_ = MaybePoint::Infinity;

    for (_, (hiding_nonce, binding_nonce)) in sorted_nonces {
        hiding_agg_nonce_ = hiding_agg_nonce_ + hiding_nonce.to_owned();
        binding_agg_nonce_ = binding_agg_nonce_ + binding_nonce.to_owned();
    }

    let hiding_agg_nonce = match hiding_agg_nonce_ {
        MaybePoint::Valid(point) => point,
        MaybePoint::Infinity => return None,
    };

    let binding_agg_nonce = match binding_agg_nonce_ {
        MaybePoint::Valid(point) => point,
        MaybePoint::Infinity => return None,
    };

    Some((hiding_agg_nonce, binding_agg_nonce))
}
