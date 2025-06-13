use crate::constructive::taproot::{TapRoot, P2TR};
use crate::transmutative::musig::keyagg::MusigKeyAggCtx;
use secp::Point;
use serde::{Deserialize, Serialize};

type Bytes = Vec<u8>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Connector {
    remote: Point,
    operator: Point,
}

impl Connector {
    pub fn new(remote: Point, operator: Point) -> Connector {
        Connector { remote, operator }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn operator_key(&self) -> Point {
        self.operator
    }

    pub fn remote_key(&self) -> Point {
        self.remote.clone()
    }

    pub fn keys(&self) -> Vec<Point> {
        let mut keys = Vec::<Point>::new();

        keys.push(self.operator_key());
        keys.push(self.remote_key());

        keys
    }

    pub fn agg_inner_key(&self) -> Option<Point> {
        let keys = self.keys();
        let key_agg_ctx = MusigKeyAggCtx::new(&keys, None)?;
        let agg_inner_key = key_agg_ctx.agg_inner_key();

        Some(agg_inner_key)
    }

    pub fn key_agg_ctx(&self) -> Option<MusigKeyAggCtx> {
        let keys = self.keys();
        let key_agg_ctx = MusigKeyAggCtx::new(&keys, None)?;

        Some(key_agg_ctx)
    }
}

impl P2TR for Connector {
    fn taproot(&self) -> Option<TapRoot> {
        //// Inner Key: (Self + Operator)
        let agg_inner_key = self.agg_inner_key()?;

        Some(TapRoot::key_path_only(agg_inner_key))
    }

    fn spk(&self) -> Option<Bytes> {
        self.taproot()?.spk()
    }
}
