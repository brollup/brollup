use crate::{
    hash::{Hash, HashTag},
    schnorr::Sighash,
    valtype::account::Account,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Move {
    from: Account,
    to: Account,
    amount: u32,
}

impl Move {
    pub fn new(from: Account, to: Account, amount: u32) -> Move {
        Move { from, to, amount }
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

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn validate_account(&self, account: Account) -> bool {
        self.from.key() == account.key()
    }
}

impl Sighash for Move {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        preimage.extend(self.from.key().serialize_xonly());
        preimage.extend(self.to.key().serialize_xonly());
        preimage.extend(self.amount.to_le_bytes());

        preimage.hash(Some(HashTag::SighashCombinator))
    }
}
