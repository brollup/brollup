use crate::{hash::Hash, into::IntoScalar, schnorr::challenge};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct MusigCtx {
    pub signers: HashMap<Point, (Point, Point)>,
    pub key_coef: Scalar,
    pub nonce_coef: Scalar,
    pub agg_key: Point,
    pub agg_nonce: Point,
    pub message: [u8; 32],
    pub challenge: Scalar,
    partial_sigs: Vec<Scalar>,
}

impl MusigCtx {
    pub fn new(signers: HashMap<Point, (Point, Point)>, message: [u8; 32]) -> Option<Self> {
        let keys: Vec<Point> = {
            let mut keys: Vec<Point> = signers.keys().cloned().collect();
            keys.sort();
            keys
        };

        let nonces: Vec<(Point, Point)> = {
            let mut key_value_pairs: Vec<_> = signers.iter().collect();
            key_value_pairs.sort_by_key(|(key, _)| *key); // Sort by the keys
            key_value_pairs
                .into_iter()
                .map(|(_, value)| value.clone())
                .collect()
        };

        let key_coef = key_coef(&keys)?;
        let agg_key = agg_key(key_coef, &keys)?;
        let nonce_coef = nonce_coef(&keys, &nonces, message)?;
        let agg_nonce = agg_nonce(nonce_coef, &nonces)?;
        let challenge = compute_challenge(agg_nonce, agg_key, message)?;

        let musig_ctx = MusigCtx {
            signers,
            key_coef,
            nonce_coef,
            agg_key,
            agg_nonce,
            message,
            challenge,
            partial_sigs: Vec::<Scalar>::new(),
        };

        Some(musig_ctx)
    }

    pub fn agg_key(&self) -> Point {
        self.agg_key.clone()
    }

    pub fn agg_nonce(&self) -> Point {
        self.agg_nonce.clone()
    }

    pub fn keys(&self) -> Vec<Point> {
        let mut keys: Vec<Point> = self.signers.keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn nonces(&self) -> Vec<(Point, Point)> {
        let mut key_value_pairs: Vec<_> = self.signers.iter().collect();
        key_value_pairs.sort_by_key(|(key, _)| *key); // Sort by the keys
        key_value_pairs
            .into_iter()
            .map(|(_, value)| value.clone())
            .collect()
    }

    pub fn partial_sign(
        &self,
        signatory: Point,
        secret_key: Scalar,
        secret_hiding_nonce: Scalar,
        secet_binding_nonce: Scalar,
    ) -> Option<Scalar> {
        if secret_key.base_point_mul() != signatory {
            return None;
        };

        let (hiding_public_nonce, binding_public_nonce) = match self.signers.get(&signatory) {
            Some(tuple) => tuple,
            None => return None,
        };

        if secret_hiding_nonce.base_point_mul() != hiding_public_nonce.to_owned() {
            return None;
        };

        if secet_binding_nonce.base_point_mul() != binding_public_nonce.to_owned() {
            return None;
        };

        let secret_key = secret_key.negate_if(self.agg_key.parity());
        let secret_hiding_nonce = secret_hiding_nonce.negate_if(self.agg_nonce.parity());
        let secet_binding_nonce = secet_binding_nonce.negate_if(self.agg_nonce.parity());
        // k + k + ed

        let partial_sig = match secret_hiding_nonce
            + (secet_binding_nonce * self.nonce_coef)
            + (secret_key * self.key_coef * self.challenge)
        {
            MaybeScalar::Valid(scalar) => scalar,
            MaybeScalar::Zero => return None,
        };

        Some(partial_sig)
    }

    pub fn insert_partial_sig(&mut self, signatory: Point, partial_sig: Scalar) -> bool {
        if self.partial_sigs.contains(&partial_sig) {
            return false;
        }

        let (hiding_nonce, binding_nonce) = match self.signers.get(&signatory) {
            Some(tuple) => tuple,
            None => return false,
        };

        let signatory = signatory.negate_if(self.agg_key.parity());
        let hiding_nonce = hiding_nonce.negate_if(self.agg_nonce.parity());
        let binding_nonce = binding_nonce.negate_if(self.agg_nonce.parity());

        let eq = match hiding_nonce.to_owned()
            + (binding_nonce.to_owned() * self.nonce_coef)
            + signatory * self.key_coef * self.challenge
        {
            MaybePoint::Valid(point) => point,
            MaybePoint::Infinity => return false,
        };

        if eq != partial_sig.base_point_mul() {
            return false;
        };

        self.partial_sigs.push(partial_sig);

        true
    }

    pub fn agg_sig(&self) -> Option<Scalar> {
        if self.partial_sigs.len() != self.signers.len() {
            return None;
        };

        let mut agg_sig = MaybeScalar::Zero;

        for partial_sig in self.partial_sigs.iter() {
            agg_sig = agg_sig + partial_sig.to_owned();
        }

        match agg_sig {
            MaybeScalar::Valid(scalar) => return Some(scalar),
            MaybeScalar::Zero => return None,
        }
    }

    pub fn full_agg_sig(&self) -> Option<[u8; 64]> {
        let mut full_agg_sig = Vec::<u8>::with_capacity(64);
        full_agg_sig.extend(self.agg_nonce.serialize_xonly());
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
}

impl MusigNestingCtx {
    pub fn new(remote: HashMap<Point, (Point, Point)>) -> Self {
        MusigNestingCtx { remote }
    }

    pub fn musig_ctx(
        &self,
        operator_key: Point,
        operator_hiding_nonce: Point,
        operator_post_binding_nonce: Point,
        message: [u8; 32],
    ) -> Option<MusigCtx> {
        let mut signers = self.remote.clone();
        signers.insert(
            operator_key,
            (operator_hiding_nonce, operator_post_binding_nonce),
        );

        MusigCtx::new(signers, message)
    }
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

fn nonce_coef(
    keys: &Vec<Point>,
    nonces: &Vec<(Point, Point)>,
    message: [u8; 32],
) -> Option<Scalar> {
    let mut coef_preimage = Vec::<u8>::new();
    coef_preimage.extend(message);

    for key in keys {
        coef_preimage.extend(key.serialize());
    }

    for (hiding, binding) in nonces {
        coef_preimage.extend(hiding.serialize());
        coef_preimage.extend(binding.serialize());
    }

    coef_preimage.hash(None).into_scalar().ok()
}

fn key_coef(keys: &Vec<Point>) -> Option<Scalar> {
    let mut coef_preimage = Vec::<u8>::new();

    for key in keys {
        coef_preimage.extend(key.serialize());
    }

    coef_preimage.hash(None).into_scalar().ok()
}

fn agg_nonce(nonce_coef: Scalar, nonces: &Vec<(Point, Point)>) -> Option<Point> {
    let mut agg_hiding_point = MaybePoint::Infinity;
    let mut agg_binding_point = MaybePoint::Infinity;

    for (hiding, binding) in nonces {
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
