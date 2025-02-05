use super::{keyagg::MusigKeyAggCtx, session::MusigSessionCtx};
use secp::{Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    ) -> Option<MusigSessionCtx> {
        let mut nonces = self.remote.clone();
        nonces.insert(
            operator_key,
            (operator_hiding_nonce, operator_post_binding_nonce),
        );

        let keys: Vec<Point> = nonces.keys().cloned().collect();

        let key_agg_ctx = MusigKeyAggCtx::new(&keys, self.tweak())?;

        let mut musig_ctx = MusigSessionCtx::new(&key_agg_ctx, message)?;

        for (key, (hiding_nonce, binding_nonce)) in nonces {
            if !musig_ctx.insert_nonce(key, hiding_nonce, binding_nonce) {
                return None;
            }
        }

        Some(musig_ctx)
    }
}
