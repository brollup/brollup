use crate::{
    into::IntoPoint,
    musig::{MusigCtx, MusigNestingCtx},
    COV_SESSION,
};
use secp::{Point, Scalar};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CovSessionStage {
    Close,
    On,        // collect keys, hiding and binding nonces.
    Locked,    // no longer accepting remote. MusigNestingCtx ready.
    Ready,     // full musig conetxt is ready
    Finalized, // collected all partial sigs.
}

pub struct CovSession {
    stage: CovSessionStage,
    remote: HashMap<Point, (Point, Point)>,
    musig_nesting_ctx: Option<MusigNestingCtx>,
    musig_ctx: Option<MusigCtx>,
}

impl CovSession {
    pub fn construct() -> COV_SESSION {
        let session = CovSession {
            stage: CovSessionStage::Close,
            remote: HashMap::<Point, (Point, Point)>::new(),
            musig_nesting_ctx: None,
            musig_ctx: None,
        };
        Arc::new(Mutex::new(session))
    }

    pub fn stage(&self) -> CovSessionStage {
        self.stage
    }

    pub fn remote(&self) -> HashMap<Point, (Point, Point)> {
        self.remote.clone()
    }

    pub fn musig_nesting_ctx(&self) -> Option<MusigNestingCtx> {
        self.musig_nesting_ctx.clone()
    }

    pub fn musig_ctx(&self) -> Option<MusigCtx> {
        self.musig_ctx.clone()
    }

    pub fn on(&mut self) {
        self.stage = CovSessionStage::On;
        self.remote = HashMap::<Point, (Point, Point)>::new();
        self.musig_nesting_ctx = None;
    }

    pub fn add_remote(&mut self, key: [u8; 32], hiding_nonce: Point, binding_nonce: Point) -> bool {
        if self.stage != CovSessionStage::On {
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
        self.stage = CovSessionStage::Locked;

        let musig_nesting_ctx = MusigNestingCtx::new(self.remote.clone());
        self.musig_nesting_ctx = Some(musig_nesting_ctx);

        true
    }

    pub fn ready(&mut self, musig_ctx: &MusigCtx) {
        self.stage = CovSessionStage::Ready;
        self.musig_nesting_ctx = None; // save space.
        self.musig_ctx = Some(musig_ctx.to_owned());
    }

    pub fn insert_partial_sig(&mut self, key: [u8; 32], partial_sig: Scalar) -> bool {
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
        self.stage = CovSessionStage::Finalized;
    }
}

// HashMap<Point, (Point, Point)>,
