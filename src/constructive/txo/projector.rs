use crate::constructive::taproot::{TapLeaf, TapRoot, P2TR};
use crate::transmutative::codec::csv::{CSVEncode, CSVFlag};
use crate::transmutative::musig::keyagg::MusigKeyAggCtx;
use crate::transmutative::secp::into::IntoScalar;
use secp::Point;
use serde::Deserialize;
use serde::Serialize;

type Bytes = Vec<u8>;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ProjectorTag {
    VTXOProjector,
    ConnectorProjector,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Projector {
    remote: Vec<Point>,
    operator: Point,
    tag: ProjectorTag,
}

impl Projector {
    pub fn new(remote: &Vec<Point>, operator: Point, tag: ProjectorTag) -> Projector {
        Projector {
            remote: remote.to_owned(),
            operator,
            tag,
        }
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

    pub fn remote_keys(&self) -> Vec<Point> {
        self.remote.clone()
    }

    pub fn keys(&self) -> Vec<Point> {
        let mut keys = self.remote_keys();
        keys.push(self.operator_key());

        keys
    }

    pub fn tag(&self) -> ProjectorTag {
        self.tag
    }

    pub fn agg_inner_key(&self) -> Option<Point> {
        let keys = self.keys();
        let key_agg_ctx = MusigKeyAggCtx::new(&keys, None)?;
        let agg_inner_key = key_agg_ctx.agg_inner_key();

        Some(agg_inner_key)
    }

    pub fn key_agg_ctx(&self) -> Option<MusigKeyAggCtx> {
        let taproot = self.taproot()?;
        let tweak = taproot.tap_tweak().into_scalar().ok()?;
        let keys = self.keys();
        let key_agg_ctx = MusigKeyAggCtx::new(&keys, Some(tweak))?;

        Some(key_agg_ctx)
    }
}

impl P2TR for Projector {
    fn taproot(&self) -> Option<TapRoot> {
        //// Inner Key: (Self + Operator)
        let agg_inner_key = self.agg_inner_key()?;

        //// Sweep Path: (Operator after 3 months)
        let mut sweep_path_script = Vec::<u8>::new();
        sweep_path_script.extend(Bytes::csv_script(CSVFlag::CSVThreeMonths)); // Relative Timelock
        sweep_path_script.push(0x20); // OP_PUSHDATA_32
        sweep_path_script.extend(self.operator_key().serialize_xonly()); // Operator Key 32-bytes
        sweep_path_script.push(0xac); // OP_CHECKSIG

        let sweep_path = TapLeaf::new(sweep_path_script);

        Some(TapRoot::key_and_script_path_single(
            agg_inner_key,
            sweep_path,
        ))
    }

    fn spk(&self) -> Option<Bytes> {
        self.taproot()?.spk()
    }
}
