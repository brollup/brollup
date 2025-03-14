use crate::encoding::csv::CSVEncode;
use crate::encoding::csv::CSVFlag;
use crate::musig::keyagg::MusigKeyAggCtx;
use crate::taproot::{TapLeaf, TapRoot};
use crate::txn::outpoint::Outpoint;
use crate::{into::IntoScalar, taproot::P2TR};
use secp::Point;
use serde::{Deserialize, Serialize};

type Bytes = Vec<u8>;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VTXO {
    account: Point,
    operator: Point,
    outpoint: Option<Outpoint>,
    value: Option<u64>,
}

impl VTXO {
    pub fn new(
        account: Point,
        operator: Point,
        outpoint: Option<Outpoint>,
        value: Option<u64>,
    ) -> VTXO {
        VTXO {
            account,
            operator,
            outpoint,
            value,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn account_key(&self) -> Point {
        self.account.clone()
    }

    pub fn operator_key(&self) -> Point {
        self.operator
    }

    pub fn outpoint(&self) -> Option<Outpoint> {
        self.outpoint
    }

    pub fn value(&self) -> Option<u64> {
        self.value
    }

    pub fn keys(&self) -> Vec<Point> {
        let mut keys = Vec::<Point>::new();

        keys.push(self.account_key());
        keys.push(self.operator_key());

        keys
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

impl P2TR for VTXO {
    fn taproot(&self) -> Option<TapRoot> {
        //// Inner Key: (Self + Operator)
        let agg_inner_key = self.agg_inner_key()?;

        //// Sweep Path: (Operator after 3 months)
        let mut sweep_path_script = Vec::<u8>::new();
        sweep_path_script.extend(Bytes::csv_script(CSVFlag::CSVThreeMonths)); // Relative Timelock
        sweep_path_script.push(0x20); // OP_PUSHDATA_32
        sweep_path_script.extend(self.account_key().serialize_xonly()); // Account Key 32-bytes
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
