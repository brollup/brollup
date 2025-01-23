use secp::{MaybePoint, MaybeScalar, Point, Scalar};

use crate::{hash::Hash, into::IntoScalar, schnorr::challenge};

#[derive(Clone)]
pub struct MusigCtx {
    keys: Vec<Point>,
    nonces: Vec<(Point, Point)>,
    message: [u8; 32],
}

impl MusigCtx {
    pub fn new(keys: Vec<Point>, nonces: Vec<(Point, Point)>, message: [u8; 32]) -> Self {
        MusigCtx {
            keys,
            nonces,
            message,
        }
    }

    pub fn binding_coef(&self) -> Option<Scalar> {
        let mut coef_preimage = Vec::<u8>::new();
        coef_preimage.extend(self.message);

        for key in self.keys.iter() {
            coef_preimage.extend(key.serialize());
        }

        for (hiding, binding) in self.nonces.iter() {
            coef_preimage.extend(hiding.serialize());
            coef_preimage.extend(binding.serialize());
        }

        coef_preimage.hash(None).into_scalar().ok()
    }

    pub fn agg_key(&self) -> Option<Point> {
        let mut agg_point = MaybePoint::Infinity;

        for key in self.keys.iter() {
            agg_point = agg_point + key.to_owned();
        }

        match agg_point {
            MaybePoint::Valid(point) => Some(point),
            MaybePoint::Infinity => None,
        }
    }

    pub fn agg_nonce(&self) -> Option<Point> {
        let binding_coef = self.binding_coef()?;

        let mut agg_hiding_point = MaybePoint::Infinity;
        let mut agg_binding_point = MaybePoint::Infinity;

        for (hiding, binding) in self.nonces.iter() {
            agg_hiding_point = agg_hiding_point + hiding.to_owned();
            agg_binding_point = agg_binding_point + (binding.to_owned() * binding_coef);
        }

        match agg_hiding_point + agg_binding_point {
            MaybePoint::Valid(point) => Some(point),
            MaybePoint::Infinity => None,
        }
    }

    pub fn challenge(&self) -> Option<Scalar> {
        let challenge = match challenge(
            self.agg_nonce()?,
            self.agg_key()?,
            self.message,
            crate::schnorr::SigningMode::BIP340,
        ) {
            MaybeScalar::Valid(scalar) => scalar,
            MaybeScalar::Zero => return None,
        };

        Some(challenge)
    }
}

#[derive(Clone)]
pub struct MusigNestingCtx {
    remote_keys: Vec<Point>,
    remote_nonces: Vec<(Point, Point)>,
}

impl MusigNestingCtx {
    pub fn new(remote_keys: Vec<Point>, remote_nonces: Vec<(Point, Point)>) -> Self {
        MusigNestingCtx {
            remote_keys,
            remote_nonces,
        }
    }

    pub fn musig_ctx(
        &self,
        operator_key: Point,
        operator_hiding_nonce: Point,
        operator_post_binding_nonce: Point,
        message: [u8; 32],
    ) -> MusigCtx {
        let mut keys = vec![operator_key];
        keys.extend(self.remote_keys.clone());

        let mut nonces = vec![(operator_hiding_nonce, operator_post_binding_nonce)];
        nonces.extend(self.remote_nonces.clone());

        MusigCtx::new(keys, nonces, message)
    }
}
