#![allow(dead_code)]

use crate::{
    encoding::csv::{CSVEncode, CSVFlag},
    taproot::{TapLeaf, TapRoot, P2TR},
};
use musig2::secp256k1::{self, XOnlyPublicKey};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

const DEGRADING_PERIOD_START_AT: u8 = 141;

pub struct Channel {
    self_key: Key,
    operator_key_dynamic: Key,
}

impl Channel {
    pub fn new(self_key: Key, operator_key_dynamic: Key) -> Channel {
        Channel {
            self_key,
            operator_key_dynamic,
        }
    }

    pub fn to_self_key(&self) -> Key {
        self.self_key
    }

    pub fn to_operator_key(&self) -> Key {
        self.operator_key_dynamic
    }
}

impl P2TR for Channel {
    fn taproot(&self) -> Result<TapRoot, secp256k1::Error> {
        let mut leaves = Vec::<TapLeaf>::new();

        for i in 0..128 {
            let mut tap_script = Vec::<u8>::new();

            // Add degrading timelock
            let days: u8 = DEGRADING_PERIOD_START_AT - i;
            tap_script.extend(Bytes::csv_script(CSVFlag::Days(days)));

            // Push to_self key
            tap_script.push(0x20);
            tap_script.extend(self.to_self_key().serialize());

            // OP_CHECKSIGVERIFY
            tap_script.push(0xad);

            // Push to_operator key
            tap_script.push(0x20);
            tap_script.extend(self.to_operator_key().serialize());

            // OP_CHECKSIG
            tap_script.push(0xac);

            leaves.push(TapLeaf::new(tap_script));
        }

        Ok(TapRoot::script_path_only_multi(leaves))
    }

    fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot()?.spk()
    }
}
