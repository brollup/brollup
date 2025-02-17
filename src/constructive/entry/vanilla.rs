use crate::valtype::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Vanilla {
    from: Account,
    to: Account,
    amount: u32,
}

impl Vanilla {
    pub fn new(from: Account, to: Account, amount: u32) -> Vanilla {
        Vanilla { from, to, amount }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn from(&self) -> Account {
        self.from
    }

    pub fn to(&self) -> Account {
        self.to
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }
}
