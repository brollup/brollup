use crate::csv::CSVEncode;
use crate::{
    csv::CSVFlag,
    musig::keyagg,
    taproot::{TapLeaf, TapRoot, P2TR},
};
use secp::Point;

type Bytes = Vec<u8>;

#[derive(Clone, Copy)]
pub enum ProjectorTag {
    VTXOProjector,
    ConnectorProjector,
}

#[derive(Clone)]
pub struct Projector {
    remote: Vec<Point>,
    operator: Point,
    tag: ProjectorTag,
}

impl Projector {
    pub fn new(remote: Vec<Point>, operator: Point, tag: ProjectorTag) -> Projector {
        Projector {
            remote,
            operator,
            tag,
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

    pub fn agg_inner_key(&self) -> Option<Point> {
        let keys = self.keys();
        keyagg(&keys)
    }

    pub fn tag(&self) -> ProjectorTag {
        self.tag
    }
}

impl P2TR for Projector {
    fn taproot(&self) -> Option<TapRoot> {
        //// Inner Key: (Self + Operator)
        let inner_key = self.agg_inner_key()?;

        println!("projector p2tr inner_key: {}", hex::encode(inner_key.serialize()));

        //// Sweep Path: (Operator after 3 months)
        let mut sweep_path_script = Vec::<u8>::new();
        sweep_path_script.extend(Bytes::csv_script(CSVFlag::CSVThreeMonths)); // Relative Timelock
        sweep_path_script.push(0x20); // OP_PUSHDATA_32
        sweep_path_script.extend(self.operator_key().serialize_xonly()); // Operator Key 32-bytes
        sweep_path_script.push(0xac); // OP_CHECKSIG

        println!("sweep_path_script: {}", hex::encode(&sweep_path_script));

        let sweep_path = TapLeaf::new(sweep_path_script);

        Some(TapRoot::key_and_script_path_single(inner_key, sweep_path))
    }

    fn spk(&self) -> Option<Bytes> {
        self.taproot()?.spk()
    }
}
