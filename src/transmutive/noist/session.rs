use super::{
    core::lagrance::{interpolating_value, lagrance_index, lagrance_index_list},
    dkg::session::DKGSession,
};
use crate::{musig::session::MusigSessionCtx, schnorr::challenge};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};
use std::collections::HashMap;

#[derive(Clone)]
pub struct NOISTSessionCtx {
    group_key_session: DKGSession,
    group_nonce_session: DKGSession,
    group_key: Point,
    hiding_group_nonce: Point,
    post_binding_group_nonce: Point,
    group_nonce: Point,
    message: [u8; 32],
    challenge: Scalar,
    musig_ctx: Option<MusigSessionCtx>,
    partial_sigs: HashMap<Point, Scalar>,
}

impl NOISTSessionCtx {
    pub fn new(
        group_key_session: &DKGSession,
        group_nonce_session: &DKGSession,
        group_key: Point,
        hiding_group_nonce: Point,
        post_binding_group_nonce: Point,
        group_nonce: Point,
        message: [u8; 32],
        musig_ctx: Option<MusigSessionCtx>,
    ) -> Option<NOISTSessionCtx> {
        let (challenge_nonce, challenge_key) = match &musig_ctx {
            Some(ctx) => (ctx.agg_nonce()?, ctx.key_agg_ctx().agg_key()),
            None => (group_nonce, group_key),
        };

        let challenge = match challenge(
            challenge_nonce,
            challenge_key,
            message,
            crate::schnorr::SigningMode::BIP340,
        ) {
            MaybeScalar::Valid(scalar) => scalar,
            MaybeScalar::Zero => return None,
        };

        let session = NOISTSessionCtx {
            group_key_session: group_key_session.to_owned(),
            group_nonce_session: group_nonce_session.to_owned(),
            group_key,
            hiding_group_nonce,
            post_binding_group_nonce,
            group_nonce,
            message,
            challenge,
            musig_ctx,
            partial_sigs: HashMap::<Point, Scalar>::new(),
        };

        Some(session)
    }

    pub fn set_musig_ctx(&mut self, musig_ctx: &MusigSessionCtx) -> bool {
        self.musig_ctx = Some(musig_ctx.to_owned());

        let challenge_nonce = match musig_ctx.agg_nonce() {
            Some(nonce) => nonce,
            None => return false,
        };

        let challenge_key = musig_ctx.key_agg_ctx().agg_key();

        let message = self.message;

        let challenge = match challenge(
            challenge_nonce,
            challenge_key,
            message,
            crate::schnorr::SigningMode::BIP340,
        ) {
            MaybeScalar::Valid(scalar) => scalar,
            MaybeScalar::Zero => return false,
        };

        self.challenge = challenge;

        true
    }

    pub fn group_key(&self) -> Point {
        self.group_key
    }

    pub fn hiding_group_nonce(&self) -> Point {
        self.hiding_group_nonce
    }

    pub fn post_binding_group_nonce(&self) -> Point {
        self.post_binding_group_nonce
    }

    pub fn group_nonce(&self) -> Point {
        self.group_nonce
    }

    pub fn nonce_height(&self) -> u64 {
        self.group_nonce_session.index()
    }

    pub fn challenge(&self) -> Scalar {
        self.challenge
    }

    pub fn partial_sign(&self, secret_key: [u8; 32]) -> Option<Scalar> {
        let group_key_bytes = self.group_key.serialize_xonly();
        let message_bytes = self.message;
        let challenge = self.challenge;

        let compare_key = match &self.musig_ctx {
            Some(ctx) => ctx.key_agg_ctx().agg_inner_key(),
            None => self.group_key,
        };

        let compare_nonce = match &self.musig_ctx {
            Some(ctx) => ctx.agg_nonce()?,
            None => self.group_nonce,
        };

        let musig_nonce_coef = match &self.musig_ctx {
            Some(ctx) => ctx.nonce_coef()?,
            None => Scalar::one(),
        };

        let musig_key_coef = match &self.musig_ctx {
            Some(ctx) => ctx.key_agg_ctx().key_coef(self.group_key)?,
            None => Scalar::one(),
        };

        // (k + ed) + (k + ed)

        let hiding_secret_key_ = self
            .group_key_session
            .signatory_combined_hiding_secret(secret_key)?
            * musig_key_coef;

        let mut hiding_secret_key = hiding_secret_key_.negate_if(compare_key.parity());

        if let Some(ctx) = &self.musig_ctx {
            if let Some(_) = ctx.key_agg_ctx().tweak() {
                hiding_secret_key =
                    hiding_secret_key.negate_if(ctx.key_agg_ctx().agg_key().parity())
            }
        }

        let post_binding_secret_key_ = self
            .group_key_session
            .signatory_combined_post_binding_secret(secret_key, None, None)?
            * musig_key_coef;

        let mut post_binding_secret_key = post_binding_secret_key_.negate_if(compare_key.parity());

        if let Some(ctx) = &self.musig_ctx {
            if let Some(_) = ctx.key_agg_ctx().tweak() {
                post_binding_secret_key =
                    post_binding_secret_key.negate_if(ctx.key_agg_ctx().agg_key().parity())
            }
        }

        let hiding_secret_nonce_ = self
            .group_nonce_session
            .signatory_combined_hiding_secret(secret_key)?;
        let hiding_secret_nonce = hiding_secret_nonce_.negate_if(compare_nonce.parity());

        let post_binding_secret_nonce_ = self
            .group_nonce_session
            .signatory_combined_post_binding_secret(
                secret_key,
                Some(group_key_bytes),
                Some(message_bytes),
            )?
            * musig_nonce_coef;

        let post_binding_secret_nonce =
            post_binding_secret_nonce_.negate_if(compare_nonce.parity());

        let partial_sig = (hiding_secret_nonce + (challenge * hiding_secret_key))
            + (post_binding_secret_nonce + (challenge * post_binding_secret_key));

        match partial_sig {
            MaybeScalar::Valid(scalar) => return Some(scalar),
            MaybeScalar::Zero => return None,
        }
    }

    pub fn partial_sig_verify(&self, signatory: Point, sig: Scalar) -> bool {
        let group_key_bytes = self.group_key.serialize_xonly();
        let message_bytes = self.message;
        let challenge = self.challenge;

        let compare_key = match &self.musig_ctx {
            Some(ctx) => ctx.key_agg_ctx().agg_inner_key(),
            None => self.group_key,
        };

        let compare_nonce = match &self.musig_ctx {
            Some(ctx) => match ctx.agg_nonce() {
                Some(nonce) => nonce,
                None => return false,
            },
            None => self.group_nonce,
        };

        let musig_nonce_coef = match &self.musig_ctx {
            Some(ctx) => match ctx.nonce_coef() {
                Some(coef) => coef,
                None => return false,
            },
            None => Scalar::one(),
        };

        let musig_key_coef = match &self.musig_ctx {
            Some(ctx) => match ctx.key_agg_ctx().key_coef(self.group_key) {
                Some(coef) => coef,
                None => return false,
            },
            None => Scalar::one(),
        };

        // (R + eP) + (R + eP)

        let hiding_public_key_ = match self
            .group_key_session
            .signatory_combined_hiding_public(signatory)
        {
            Some(point) => point,
            None => return false,
        } * musig_key_coef;

        let mut hiding_public_key = hiding_public_key_.negate_if(compare_key.parity());

        if let Some(ctx) = &self.musig_ctx {
            if let Some(_) = ctx.key_agg_ctx().tweak() {
                hiding_public_key =
                    hiding_public_key.negate_if(ctx.key_agg_ctx().agg_key().parity())
            }
        }

        let post_binding_public_key_ = match self
            .group_key_session
            .signatory_combined_post_binding_public(signatory, None, None)
        {
            Some(point) => point,
            None => return false,
        } * musig_key_coef;

        let mut post_binding_public_key = post_binding_public_key_.negate_if(compare_key.parity());

        if let Some(ctx) = &self.musig_ctx {
            if let Some(_) = ctx.key_agg_ctx().tweak() {
                post_binding_public_key =
                    post_binding_public_key.negate_if(ctx.key_agg_ctx().agg_key().parity())
            }
        }

        let hiding_public_nonce_ = match self
            .group_nonce_session
            .signatory_combined_hiding_public(signatory)
        {
            Some(point) => point,
            None => return false,
        };

        let hiding_public_nonce = hiding_public_nonce_.negate_if(compare_nonce.parity());

        let post_binding_public_nonce_ = match self
            .group_nonce_session
            .signatory_combined_post_binding_public(
                signatory,
                Some(group_key_bytes),
                Some(message_bytes),
            ) {
            Some(point) => point,
            None => return false,
        } * musig_nonce_coef;

        let post_binding_public_nonce =
            post_binding_public_nonce_.negate_if(compare_nonce.parity());

        let equation = (hiding_public_nonce + (challenge * hiding_public_key))
            + (post_binding_public_nonce + (challenge * post_binding_public_key));

        let equation_point = match equation {
            MaybePoint::Valid(point) => point,
            MaybePoint::Infinity => return false,
        };

        equation_point == sig.base_point_mul()
    }

    pub fn insert_partial_sig(&mut self, signatory: Point, sig: Scalar) -> bool {
        if self.partial_sig_verify(signatory, sig) {
            if let None = self.partial_sigs.insert(signatory, sig) {
                return true;
            }
        }
        false
    }

    pub fn is_threshold_met(&self) -> bool {
        let threshold = (self.group_key_session.signatories().len() / 2) + 1;
        self.partial_sigs.len() >= threshold
    }

    pub fn aggregated_sig(&self) -> Option<Scalar> {
        let full_list = self.group_key_session.signatories();
        let mut active_list = Vec::<Point>::new();

        // Fill lagrance index list.
        for (signatory, _) in self.partial_sigs.iter() {
            active_list.push(signatory.to_owned());
        }

        let index_list = lagrance_index_list(&full_list, &active_list)?;

        let mut agg_sig = MaybeScalar::Zero;

        for (signatory, partial_sig) in self.partial_sigs.iter() {
            let lagrance_index = lagrance_index(&full_list, signatory.to_owned())?;
            let lagrance = interpolating_value(&index_list, lagrance_index).ok()?;
            let partial_sig_lagranced = partial_sig.to_owned() * lagrance;
            agg_sig = agg_sig + partial_sig_lagranced;
        }

        match agg_sig {
            MaybeScalar::Valid(scalar) => return Some(scalar),
            MaybeScalar::Zero => return None,
        };
    }

    pub fn full_aggregated_sig_bytes(&self) -> Option<[u8; 64]> {
        let group_nonce = self.group_nonce.serialize_xonly();
        let agg_sig = self.aggregated_sig()?.serialize();

        let mut full = Vec::<u8>::with_capacity(64);
        full.extend(group_nonce);
        full.extend(agg_sig);
        let full_bytes: [u8; 64] = full.try_into().ok()?;
        Some(full_bytes)
    }
}
