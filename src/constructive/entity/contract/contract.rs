use crate::constructive::valtype::val::short_val::short_val::ShortVal;
use serde::{Deserialize, Serialize};

/// Represents a contract; a program that can be executed on Cube.
#[derive(Clone, Copy, Serialize, Deserialize, Hash, Debug)]
pub struct Contract {
    pub contract_id: [u8; 32],
    pub registery_index: ShortVal,
    pub rank: Option<ShortVal>,
}

impl Contract {
    /// Creates a new contract.
    pub fn new(contract_id: [u8; 32], registery_index: u32, rank: Option<u32>) -> Contract {
        // Convert the registery index to a ShortVal.
        let registery_index = ShortVal::new(registery_index);

        // Convert the rank to a ShortVal.
        let rank = match rank {
            Some(rank) => Some(ShortVal::new(rank)),
            None => None,
        };

        Contract {
            contract_id,
            registery_index,
            rank,
        }
    }

    /// Returns the contract id.
    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    /// Returns the registery index.
    pub fn registery_index(&self) -> u32 {
        self.registery_index.value()
    }

    /// Returns the rank (if set).
    pub fn rank(&self) -> Option<u32> {
        self.rank.map(|rank| rank.value())
    }

    /// Sets the rank index.
    pub fn set_rank(&mut self, rank: Option<u32>) {
        self.rank = rank.map(|rank| ShortVal::new(rank));
    }

    /// Serializes the contract.
    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }
}

impl PartialEq for Contract {
    fn eq(&self, other: &Self) -> bool {
        self.contract_id == other.contract_id
    }
}

impl Eq for Contract {}
