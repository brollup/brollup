#![allow(dead_code)]

use crate::{
    signature::keyagg::KeyAgg,
    taproot::{TapRoot, P2TR},
    well_known::operator,
};
use musig2::{
    secp256k1::{self, XOnlyPublicKey},
    KeyAggContext,
};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

pub struct Connector {
    self_key: Key,
    operator_key_well_known: Key,
}

impl Connector {
    pub fn new(self_key: Key) -> Connector {
        let operator_key_well_known = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();
        Connector {
            self_key,
            operator_key_well_known,
        }
    }

    pub fn new_with_operator(self_key: Key, operator_key_well_known: Key) -> Connector {
        Connector {
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
            .agg_key(None)
            .map_err(|_| secp256k1::Error::InvalidPublicKey)?;

        Ok(agg_inner_key)
    }

    pub fn key_agg_ctx(&self) -> Result<KeyAggContext, secp256k1::Error> {
        let keys = self.keys();

        let key_agg_ctx = keys
            .key_agg_ctx(None)
            .map_err(|_| secp256k1::Error::InvalidPublicKey)?;

        Ok(key_agg_ctx)
    }
}

impl P2TR for Connector {
    fn taproot(&self) -> Result<TapRoot, secp256k1::Error> {
        //// Inner Key: (Self + Operator)
        let inner_key = self.agg_inner_key()?;

        Ok(TapRoot::key_path_only(inner_key))
    }

    fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot()?.spk()
    }
}
