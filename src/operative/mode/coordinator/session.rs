use crate::{
    into::{IntoPoint, IntoScalar},
    musig::{keyagg::MusigKeyAggCtx, session::MusigSessionCtx},
    SESSION_CTX,
};
use secp::{Point, Scalar};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SessionStage {
    Close,
    On,        // collect keys, hiding and binding nonces.
    Locked,    // no longer accepting remote. MusigNestingCtx ready.
    Ready,     // full musig conetxt is ready
    Finalized, // collected all partial sigs.
}

pub struct SessionCtx {
    stage: SessionStage,
    remote: HashMap<Point, (Point, Point)>,
    musig_ctx: Option<MusigSessionCtx>,
}

impl SessionCtx {
    pub fn construct() -> SESSION_CTX {
        let session = SessionCtx {
            stage: SessionStage::Close,
            remote: HashMap::<Point, (Point, Point)>::new(),
            musig_ctx: None,
        };
        Arc::new(Mutex::new(session))
    }

    pub fn stage(&self) -> SessionStage {
        self.stage
    }

    pub fn remote_len(&self) -> usize {
        self.remote.len()
    }

    pub fn remote(&self) -> HashMap<Point, (Point, Point)> {
        self.remote.clone()
    }

    pub fn remote_keys(&self) -> Vec<Point> {
        self.remote.keys().cloned().collect()
    }

    fn tweak(&self) -> Scalar {
        Scalar::one()
    }

    fn message(&self) -> [u8; 32] {
        [0xffu8; 32]
    }

    pub fn set_musig_ctx(
        &mut self,
        operator_key: Point,
        operator_hiding_nonce: Point,
        operator_binding_nonce: Point,
    ) -> Option<MusigSessionCtx> {
        let mut keys = Vec::<Point>::new();
        keys.extend(&self.remote_keys());
        keys.push(operator_key);

        let key_agg_ctx = MusigKeyAggCtx::new(&self.remote_keys(), Some(self.tweak()))?;
        let mut musig_ctx = MusigSessionCtx::new(&key_agg_ctx, self.message())?;

        for (key, (hiding_nonce, binding_nonce)) in self.remote() {
            if !musig_ctx.insert_nonce(key, hiding_nonce, binding_nonce) {
                return None;
            }
        }

        if !musig_ctx.insert_nonce(operator_key, operator_hiding_nonce, operator_binding_nonce) {
            return None;
        }

        self.musig_ctx = Some(musig_ctx.clone());

        Some(musig_ctx)
    }

    pub fn musig_ctx(&self) -> Option<MusigSessionCtx> {
        self.musig_ctx.clone()
    }

    pub fn on(&mut self) {
        self.stage = SessionStage::On;
        self.remote = HashMap::<Point, (Point, Point)>::new();
    }

    pub fn add_remote(&mut self, key: [u8; 32], hiding_nonce: Point, binding_nonce: Point) -> bool {
        if self.stage != SessionStage::On {
            return false;
        };

        let key_point = match key.into_point() {
            Ok(point) => point,
            Err(_) => return false,
        };

        match self.remote.insert(key_point, (hiding_nonce, binding_nonce)) {
            None => true,
            Some(_) => false,
        }
    }

    pub fn lock(&mut self) -> bool {
        self.stage = SessionStage::Locked;

        let tweak = [0xfeu8; 32].into_scalar().unwrap();

        true
    }

    pub fn ready(&mut self, musig_ctx: &MusigSessionCtx) {
        self.stage = SessionStage::Ready;

        self.musig_ctx = Some(musig_ctx.to_owned());
    }

    pub fn insert_partial_sig(&mut self, key: [u8; 32], partial_sig: Scalar) -> bool {
        if self.stage != SessionStage::Ready {
            return false;
        };

        let key_point = match key.into_point() {
            Ok(point) => point,
            Err(_) => return false,
        };

        if !self.remote.contains_key(&key_point) {
            return false;
        }

        match &mut self.musig_ctx {
            Some(ctx) => ctx.insert_partial_sig(key_point, partial_sig),
            None => false,
        }
    }

    pub fn full_agg_sig(&self) -> Option<[u8; 64]> {
        let musig_ctx = match &self.musig_ctx {
            Some(ctx) => ctx,
            None => return None,
        };

        musig_ctx.full_agg_sig()
    }

    pub fn finalized(&mut self) {
        self.stage = SessionStage::Finalized;
    }

    pub fn reset(&mut self) {
        self.stage = SessionStage::Close;
        self.remote = HashMap::<Point, (Point, Point)>::new();
        self.musig_ctx = None;
    }
}

// HashMap<Point, (Point, Point)>,
