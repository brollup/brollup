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

pub struct Lift {
    self_key: Key,
    operator_key_well_known: Key,
}

impl Lift {
    pub fn new(self_key: Key) -> Lift {
        let operator_key_well_known = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();
        Lift {
            self_key,
            operator_key_well_known,
        }
    }

    pub fn new_with_operator(self_key: Key, operator_key_well_known: Key) -> Lift {
        Lift {
            self_key,
            operator_key_well_known,
        }
    }

    pub fn self_key(&self) -> Key {
        self.self_key
    }

    pub fn operator_key(&self) -> Key {
        self.operator_key_well_known
    }

    pub fn keys(&self) -> Vec<Key> {
        vec![self.self_key(), self.operator_key()]
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
}

impl P2TR for Lift {
    fn taproot(&self) -> Result<TapRoot, secp256k1::Error> {
        // Inner Key: (Self + Operator)
        let inner_key = self.agg_inner_key()?;

        // Exit Path: (Self after 3 months)
        let mut exit_path_script = Vec::<u8>::new();
        exit_path_script.extend(Bytes::csv_script(CSVFlag::CSVYear)); // Relative Timelock
        exit_path_script.push(0x20); // OP_PUSHDATA_32
        exit_path_script.extend(self.self_key().serialize()); // Self Key 32-bytes
        exit_path_script.push(0xac); // OP_CHECKSIG

        let exit_path = TapLeaf::new(exit_path_script);

        Ok(TapRoot::key_and_script_path_single(inner_key, exit_path))
    }

    fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot()?.spk()
    }
}
