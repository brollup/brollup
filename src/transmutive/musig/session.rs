use super::keyagg::MusigKeyAggCtx;
use crate::{
    hash::{Hash, HashTag},
    into::IntoScalar,
    schnorr::challenge,
};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct MusigSessionCtx {
    key_agg_ctx: MusigKeyAggCtx,
    message: [u8; 32],
    nonces: HashMap<Point, (Point, Point)>,
    nonce_coef: Option<Scalar>,
    agg_nonce: Option<Point>,
    challenge: Option<Scalar>,
    partial_sigs: HashMap<Point, Scalar>,
}

impl MusigSessionCtx {
    pub fn new(key_agg_ctx: &MusigKeyAggCtx, message: [u8; 32]) -> Option<Self> {
        let ctx = MusigSessionCtx {
            key_agg_ctx: key_agg_ctx.to_owned(),
            message,
            nonces: HashMap::<Point, (Point, Point)>::new(),
            nonce_coef: None,
            agg_nonce: None,
            challenge: None,
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

        if self.key_agg_ctx.num_keys() == self.nonces.len() {
            self.set_values();
        }

        true
    }

    fn set_values(&mut self) {
        let (hiding_agg_nonce, binding_agg_nonce) = match pre_nonce_agg(&self.nonces) {
            Some(nonce) => nonce,
            None => return,
        };

        let nonce_coef = match nonce_coef(
            hiding_agg_nonce,
            binding_agg_nonce,
            self.key_agg_ctx.agg_key(),
            self.message,
        ) {
            Some(coef) => coef,
            None => return,
        };

        if let None = self.nonce_coef {
            self.nonce_coef = Some(nonce_coef)
        };

        let agg_nonce = match hiding_agg_nonce + (binding_agg_nonce * nonce_coef) {
            MaybePoint::Valid(point) => point,
            MaybePoint::Infinity => return,
        };

        if let None = self.agg_nonce {
            self.agg_nonce = Some(agg_nonce)
        };

        let challenge = match compute_challenge(agg_nonce, self.key_agg_ctx.agg_key(), self.message)
        {
            Some(challenge) => challenge,
            None => return,
        };

        if let None = self.challenge {
            self.challenge = Some(challenge)
        };
    }

    pub fn key_agg_ctx(&self) -> MusigKeyAggCtx {
        self.key_agg_ctx.clone()
    }

    pub fn nonce_coef(&self) -> Option<Scalar> {
        self.nonce_coef
    }

    pub fn message(&self) -> [u8; 32] {
        self.message
    }

    pub fn challenge(&self) -> Option<Scalar> {
        self.challenge
    }

    pub fn ready(&mut self) -> bool {
        self.key_agg_ctx.num_keys() == self.nonces.len()
    }

    pub fn agg_nonce(&self) -> Option<Point> {
        self.agg_nonce
    }

    pub fn partial_sign(
        &self,
        secret_key: Scalar,
        secret_hiding_nonce: Scalar,
        secet_binding_nonce: Scalar,
    ) -> Option<Scalar> {
        let public_key = secret_key.base_point_mul();
        let key_coef = match self.key_agg_ctx.key_coef(public_key) {
            Some(coef) => coef,
            None => return None,
        };

        let (hiding_public_nonce, binding_public_nonce) = match self.nonces.get(&public_key) {
            Some(tuple) => tuple,
            None => return None,
        };

        if secret_hiding_nonce.base_point_mul() != hiding_public_nonce.to_owned() {
            return None;
        };

        if secet_binding_nonce.base_point_mul() != binding_public_nonce.to_owned() {
            return None;
        };

        let mut secret_key = secret_key.negate_if(self.key_agg_ctx.agg_inner_key().parity());

        if let Some(_) = self.key_agg_ctx.tweak() {
            secret_key = secret_key.negate_if(self.key_agg_ctx.agg_key().parity());
        }

        let challenge = match self.challenge {
            Some(challenge) => challenge,
            None => return None,
        };

        let nonce_coef = self.nonce_coef?;
        let agg_nonce = self.agg_nonce?;

        let secret_hiding_nonce = secret_hiding_nonce.negate_if(agg_nonce.parity());
        let secet_binding_nonce = secet_binding_nonce.negate_if(agg_nonce.parity());
        // k + k + ed

        let partial_sig = match secret_hiding_nonce
            + (secet_binding_nonce * nonce_coef)
            + (secret_key * key_coef * challenge)
        {
            MaybeScalar::Valid(scalar) => scalar,
            MaybeScalar::Zero => return None,
        };

        Some(partial_sig)
    }

    pub fn insert_partial_sig(&mut self, key: Point, partial_sig: Scalar) -> bool {
        if let Some(_) = self.partial_sigs.get(&key) {
            return false;
        }

        let key_coef = match self.key_agg_ctx.key_coef(key) {
            Some(coef) => coef,
            None => return false,
        };

        let (hiding_public_nonce, binding_public_nonce) = match self.nonces.get(&key) {
            Some(tuple) => tuple,
            None => return false,
        };

        let mut key = key.negate_if(self.key_agg_ctx.agg_inner_key().parity());

        if let Some(_) = self.key_agg_ctx.tweak() {
            key = key.negate_if(self.key_agg_ctx.agg_key().parity());
        }

        let nonce_coef = match self.nonce_coef {
            Some(coef) => coef,
            None => return false,
        };

        let agg_nonce = match self.agg_nonce {
            Some(nonce) => nonce,
            None => return false,
        };

        let hiding_public_nonce = hiding_public_nonce.negate_if(agg_nonce.parity());
        let binding_public_nonce = binding_public_nonce.negate_if(agg_nonce.parity());

        let challenge = match self.challenge {
            Some(challenge) => challenge,
            None => return false,
        };

        let eq = match hiding_public_nonce.to_owned()
            + (binding_public_nonce.to_owned() * nonce_coef)
            + key * key_coef * challenge
        {
            MaybePoint::Valid(point) => point,
            MaybePoint::Infinity => return false,
        };

        if eq != partial_sig.base_point_mul() {
            return false;
        };

        self.partial_sigs.insert(key, partial_sig);

        true
    }

    pub fn blame_list(&self) -> Vec<Point> {
        let mut blame_list = Vec::<Point>::new();

        for key in self.key_agg_ctx.keys().iter() {
            if let None = self.partial_sigs.get(&key) {
                blame_list.push(key.to_owned());
            }
        }

        blame_list
    }

    pub fn agg_sig(&self) -> Option<Scalar> {
        if self.blame_list().len() != 0 {
            return None;
        }

        let mut agg_sig = MaybeScalar::Zero;

        for (_, partial_sig) in self.partial_sigs.iter() {
            agg_sig = agg_sig + partial_sig.to_owned();
        }

        let challenge = self.challenge?;

        if let Some(tweak) = self.key_agg_ctx.tweak() {
            let parity: bool = self.key_agg_ctx.agg_key().parity().into();

            if parity {
                agg_sig = agg_sig + (challenge * tweak) * Scalar::max()
            } else {
                agg_sig = agg_sig + (challenge * tweak)
            }
        }

        match agg_sig {
            MaybeScalar::Valid(scalar) => return Some(scalar),
            MaybeScalar::Zero => return None,
        }
    }

    pub fn full_agg_sig(&self) -> Option<[u8; 64]> {
        let agg_nonce = self.agg_nonce?;

        let mut full_agg_sig = Vec::<u8>::with_capacity(64);
        full_agg_sig.extend(agg_nonce.serialize_xonly());
        full_agg_sig.extend(self.agg_sig()?.serialize());

        let sig: [u8; 64] = match full_agg_sig.try_into() {
            Ok(sig) => sig,
            Err(_) => return None,
        };

        Some(sig)
    }
}

fn compute_challenge(agg_nonce: Point, agg_key: Point, message: [u8; 32]) -> Option<Scalar> {
    let challenge = match challenge(
        agg_nonce,
        agg_key,
        message,
        crate::schnorr::SigningMode::BIP340,
    ) {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => return None,
    };

    Some(challenge)
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
