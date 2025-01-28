use crate::{hash::Hash, into::IntoScalar, schnorr::challenge};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct MusigCtx {
    // Initialization
    keys: Vec<Point>,
    key_coef: Scalar,
    agg_inner_key: Point,
    tweak: Option<Scalar>,
    agg_key: Point,
    // Set message
    message: Option<[u8; 32]>,
    // Insert nonces
    nonces: HashMap<Point, (Point, Point)>, // Hiding and binding nonces.
    // Fill sigs
    partial_sigs: HashMap<Point, Scalar>,
}

impl MusigCtx {
    pub fn new(keys: &Vec<Point>, tweak: Option<Scalar>) -> Option<Self> {
        let mut keys = keys.clone();
        keys.sort();

        let key_coef = key_coef(&keys)?;
        let agg_inner_key = agg_key(key_coef, &keys)?;

        let agg_key = match tweak {
            Some(tweak) => {
                match agg_inner_key.negate_if(agg_inner_key.parity()) + tweak.base_point_mul() {
                    MaybePoint::Valid(point) => point,
                    MaybePoint::Infinity => return None,
                }
            }
            None => agg_inner_key.clone(),
        };

        let ctx = MusigCtx {
            keys: keys.clone(),
            key_coef,
            agg_inner_key,
            tweak,
            agg_key,
            message: None,
            nonces: HashMap::<Point, (Point, Point)>::new(),
            partial_sigs: HashMap::<Point, Scalar>::new(),
        };

        Some(ctx)
    }

    pub fn keys(&self) -> Vec<Point> {
        self.keys.clone()
    }

    pub fn tweak(&self) -> Option<Scalar> {
        self.tweak.clone()
    }

    pub fn key_coef(&self) -> Scalar {
        self.key_coef
    }

    pub fn agg_inner_key(&self) -> Point {
        self.agg_inner_key.clone()
    }

    pub fn agg_key(&self) -> Point {
        self.agg_key.clone()
    }

    pub fn set_message(&mut self, message: [u8; 32]) {
        self.message = Some(message);
    }

    pub fn insert_nonce(&mut self, key: Point, hiding_nonce: Point, binding_nonce: Point) -> bool {
        if !self.keys.contains(&key) {
            return false;
        }

        if let Some(_) = self.nonces.insert(key, (hiding_nonce, binding_nonce)) {
            return false;
        }

        true
    }

    pub fn nonces_ready(&self) -> bool {
        self.keys.len() == self.nonces.len()
    }

    pub fn nonce_coef(&self) -> Option<Scalar> {
        if !self.nonces_ready() {
            return None;
        }
        let message = self.message?;
        nonce_coef(&self.nonces, message)
    }

    pub fn agg_nonce(&self) -> Option<Point> {
        let nonce_coef = self.nonce_coef()?;
        agg_nonce(nonce_coef, &self.nonces)
    }

    pub fn challenge(&self) -> Option<Scalar> {
        let message = self.message?;
        let agg_key = self.agg_key();
        let agg_nonce = self.agg_nonce()?;

        compute_challenge(agg_nonce, agg_key, message)
    }

    pub fn partial_sign(
        &self,
        key: Point,
        secret_key: Scalar,
        secret_hiding_nonce: Scalar,
        secet_binding_nonce: Scalar,
    ) -> Option<Scalar> {
        if secret_key.base_point_mul() != key {
            return None;
        };

        let (hiding_public_nonce, binding_public_nonce) = match self.nonces.get(&key) {
            Some(tuple) => tuple,
            None => return None,
        };

        if secret_hiding_nonce.base_point_mul() != hiding_public_nonce.to_owned() {
            return None;
        };

        if secet_binding_nonce.base_point_mul() != binding_public_nonce.to_owned() {
            return None;
        };

        let mut secret_key = secret_key.negate_if(self.agg_inner_key.parity());

        if let Some(_) = self.tweak {
            secret_key = secret_key.negate_if(self.agg_key.parity());
        }

        let challenge = match self.challenge() {
            Some(challenge) => challenge,
            None => return None,
        };

        let nonce_coef = self.nonce_coef()?;
        let agg_nonce = self.agg_nonce()?;

        let secret_hiding_nonce = secret_hiding_nonce.negate_if(agg_nonce.parity());
        let secet_binding_nonce = secet_binding_nonce.negate_if(agg_nonce.parity());
        // k + k + ed

        let partial_sig = match secret_hiding_nonce
            + (secet_binding_nonce * nonce_coef)
            + (secret_key * self.key_coef * challenge)
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

        let (hiding_public_nonce, binding_public_nonce) = match self.nonces.get(&key) {
            Some(tuple) => tuple,
            None => return false,
        };

        let mut key = key.negate_if(self.agg_inner_key.parity());

        if let Some(_) = self.tweak {
            key = key.negate_if(self.agg_key.parity());
        }

        let nonce_coef = match self.nonce_coef() {
            Some(coef) => coef,
            None => return false,
        };

        let agg_nonce = match self.agg_nonce() {
            Some(nonce) => nonce,
            None => return false,
        };

        let hiding_public_nonce = hiding_public_nonce.negate_if(agg_nonce.parity());
        let binding_public_nonce = binding_public_nonce.negate_if(agg_nonce.parity());

        let challenge = match self.challenge() {
            Some(challenge) => challenge,
            None => return false,
        };

        let eq = match hiding_public_nonce.to_owned()
            + (binding_public_nonce.to_owned() * nonce_coef)
            + key * self.key_coef * challenge
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

    pub fn agg_sig(&self) -> Option<Scalar> {
        if self.partial_sigs.len() != self.keys.len() {
            return None;
        };

        let mut agg_sig = MaybeScalar::Zero;

        for (_, partial_sig) in self.partial_sigs.iter() {
            agg_sig = agg_sig + partial_sig.to_owned();
        }

        let challenge = self.challenge()?;

        if let Some(tweak) = self.tweak {
            let parity: bool = self.agg_key.parity().into();

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
        let agg_nonce = self.agg_nonce()?;

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

#[derive(Clone, Serialize, Deserialize)]
pub struct MusigNestingCtx {
    remote: HashMap<Point, (Point, Point)>,
    tweak: Option<Scalar>,
}

impl MusigNestingCtx {
    pub fn new(remote: HashMap<Point, (Point, Point)>, tweak: Option<Scalar>) -> Self {
        MusigNestingCtx { remote, tweak }
    }

    pub fn tweak(&self) -> Option<Scalar> {
        self.tweak.clone()
    }

    pub fn musig_ctx(
        &self,
        operator_key: Point,
        operator_hiding_nonce: Point,
        operator_post_binding_nonce: Point,
        message: [u8; 32],
    ) -> Option<MusigCtx> {
        let mut nonces = self.remote.clone();
        nonces.insert(
            operator_key,
            (operator_hiding_nonce, operator_post_binding_nonce),
        );

        let keys: Vec<Point> = nonces.keys().cloned().collect();

        let mut musig_ctx = MusigCtx::new(&keys, self.tweak())?;
        musig_ctx.set_message(message);

        for (key, (hiding_nonce, binding_nonce)) in nonces {
            if !musig_ctx.insert_nonce(key, hiding_nonce, binding_nonce) {
                return None;
            }
        }

        Some(musig_ctx)
    }
}

pub fn keyagg(keys: &Vec<Point>) -> Option<Point> {
    let key_coef = key_coef(&keys)?;
    agg_key(key_coef, &keys)
}

fn agg_key(key_coef: Scalar, keys: &Vec<Point>) -> Option<Point> {
    let mut agg_point = MaybePoint::Infinity;

    for key in keys {
        agg_point = agg_point + (key.to_owned() * key_coef);
    }

    match agg_point {
        MaybePoint::Valid(point) => Some(point),
        MaybePoint::Infinity => None,
    }
}

fn nonce_coef(nonces: &HashMap<Point, (Point, Point)>, message: [u8; 32]) -> Option<Scalar> {
    let mut coef_preimage = Vec::<u8>::new();
    coef_preimage.extend(message);

    let mut nonces_sorted: Vec<(&Point, &(Point, Point))> = nonces.iter().collect();
    nonces_sorted.sort_by_key(|(key, _)| *key);

    for (key, (hiding, binding)) in nonces_sorted {
        coef_preimage.extend(key.serialize());
        coef_preimage.extend(hiding.serialize());
        coef_preimage.extend(binding.serialize());
    }

    coef_preimage.hash(None).into_scalar().ok()
}

fn key_coef(keys: &Vec<Point>) -> Option<Scalar> {
    let mut keys = keys.clone();
    keys.sort();

    let mut coef_preimage = Vec::<u8>::new();

    for key in keys {
        coef_preimage.extend(key.serialize());
    }

    coef_preimage.hash(None).into_scalar().ok()
}

fn agg_nonce(nonce_coef: Scalar, nonces: &HashMap<Point, (Point, Point)>) -> Option<Point> {
    let mut agg_hiding_point = MaybePoint::Infinity;
    let mut agg_binding_point = MaybePoint::Infinity;

    for (_, (hiding, binding)) in nonces {
        agg_hiding_point = agg_hiding_point + hiding.to_owned();
        agg_binding_point = agg_binding_point + (binding.to_owned() * nonce_coef);
    }

    match agg_hiding_point + agg_binding_point {
        MaybePoint::Valid(point) => Some(point),
        MaybePoint::Infinity => None,
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
