#![allow(dead_code)]

use crate::{
    encoding::csv::{CSVEncode, CSVFlag},
    signature::keyagg::KeyAgg,
    taproot::{TapLeaf, TapRoot, P2TR},
    well_known::operator,
};
use musig2::{
    secp256k1::{self, XOnlyPublicKey},
    KeyAggContext,
};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

#[derive(Clone, Copy)]
pub enum ProjectorTag {
    VTXOProjector,
    ConnectorProjector,
}

#[derive(Clone)]
pub struct Projector {
    msg_sender_keys: Vec<Key>,
    operator_key_well_known: Key,
    tag: ProjectorTag,
}

impl Projector {
    pub fn new(msg_sender_keys: Vec<Key>, tag: ProjectorTag) -> Projector {
        let operator_key_well_known = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();

        Projector {
            msg_sender_keys,
            operator_key_well_known,
            tag,
        }
    }

    pub fn operator_key(&self) -> Key {
        self.operator_key_well_known
    }

    pub fn msg_sender_keys(&self) -> Vec<Key> {
        self.msg_sender_keys.clone()
    }

    pub fn keys(&self) -> Vec<Key> {
        let mut keys = self.msg_sender_keys();
        keys.push(self.operator_key());

        keys
    }

    pub fn agg_inner_key(&self) -> Result<Key, secp256k1::Error> {
        let keys = self.keys();

        let agg_inner_key = keys
            .agg_key()
            .map_err(|_| secp256k1::Error::InvalidPublicKey)?;

        Ok(agg_inner_key)
    }

    pub fn key_agg_ctx(&self) -> Result<KeyAggContext, secp256k1::Error> {
        let keys = self.keys();

        let key_agg_ctx = keys
            .key_agg_ctx()
            .map_err(|_| secp256k1::Error::InvalidPublicKey)?
            .with_taproot_tweak(&self.taproot()?.uppermost_branch())
            .map_err(|_| secp256k1::Error::InvalidTweak)?;

        Ok(key_agg_ctx)
    }

    pub fn tag(&self) -> ProjectorTag {
        self.tag
    }
}

impl P2TR for Projector {
    fn taproot(&self) -> Result<TapRoot, secp256k1::Error> {
        //// Inner Key: (Self + Operator)
        let inner_key = self.agg_inner_key()?;

        //// Sweep Path: (Operator after 3 months)
        let mut sweep_path_script = Vec::<u8>::new();
        sweep_path_script.extend(Bytes::csv_script(CSVFlag::CSVThreeMonths)); // Relative Timelock
        sweep_path_script.push(0x20); // OP_PUSHDATA_32
        sweep_path_script.extend(self.operator_key().serialize()); // Operator Key 32-bytes
        sweep_path_script.push(0xac); // OP_CHECKSIG
        let sweep_path = TapLeaf::new(sweep_path_script);

        Ok(TapRoot::key_and_script_path_single(inner_key, sweep_path))
    }

    fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot()?.spk()
    }
}
