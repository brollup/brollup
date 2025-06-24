use crate::constructive::valtype::val::short_val::short_val::ShortVal;
use secp::Point;
use serde::{Deserialize, Serialize};

/// Represents an account; a user of the system.
#[derive(Clone, Copy, Serialize, Deserialize, Hash, Debug)]
pub struct Account {
    pub key: Point,
    pub registery_index: Option<ShortVal>,
    pub rank: Option<ShortVal>,
}

impl Account {
    /// Creates a new account.
    pub fn new(key: Point, registery_index: Option<u32>, rank: Option<u32>) -> Option<Account> {
        let is_odd: bool = key.parity().into();

        if is_odd {
            return None;
        }

        // Convert the registery index to a ShortVal.
        let registery_index = match registery_index {
            Some(index) => Some(ShortVal::new(index)),
            None => None,
        };

        // Convert the rank to a ShortVal.
        let rank = match rank {
            Some(rank) => Some(ShortVal::new(rank)),
            None => None,
        };

        let account = Account {
            key,
            registery_index,
            rank,
        };

        Some(account)
    }

    /// Returns the registery index of the account.
    pub fn registery_index(&self) -> Option<u32> {
        self.registery_index.map(|index| index.value())
    }

    /// Sets the registery index of the account.
    pub fn set_registery_index(&mut self, registery_index: u32) {
        self.registery_index = Some(ShortVal::new(registery_index));
    }

    /// Returns the rank (if set).
    pub fn rank(&self) -> Option<u32> {
        self.rank.map(|rank| rank.value())
    }

    /// Sets the rank index.
    pub fn set_rank(&mut self, rank: Option<u32>) {
        self.rank = rank.map(|rank| ShortVal::new(rank));
    }

    /// Returns the key of the account.
    pub fn key(&self) -> Point {
        self.key
    }

    /// Returns true if the key is odd.
    pub fn is_odd_key(&self) -> bool {
        self.key.parity().into()
    }

    /// Serializes the account.
    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Account {}
