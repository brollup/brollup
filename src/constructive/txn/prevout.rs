use crate::txn::outpoint::Outpoint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Prevout {
    outpoint: Outpoint,
    amount: u64,
    spk: Vec<u8>,
    height: Option<u64>,
}

impl Prevout {
    pub fn new(outpoint: Outpoint, amount: u64, spk: Vec<u8>, height: Option<u64>) -> Prevout {
        Prevout {
            outpoint,
            amount,
            spk,
            height,
        }
    }

    pub fn outpoint(&self) -> Outpoint {
        self.outpoint
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn spk(&self) -> Vec<u8> {
        self.spk.clone()
    }

    pub fn height(&self) -> Option<u64> {
        self.height
    }
}
