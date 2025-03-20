use crate::{
    hash::{Hash, HashTag},
    schnorr::Sighash,
    entity::account::Account,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Remove {
    account: Account,
    amount: u32,
}

impl Remove {
    pub fn new(account: Account, amount: u32) -> Remove {
        Remove { account, amount }
    }

    pub fn account(&self) -> Account {
        self.account
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
        self.account.key() == account.key()
    }
}

impl Sighash for Remove {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        preimage.extend(self.account.key().serialize_xonly());
        preimage.extend(self.amount.to_le_bytes());

        preimage.hash(Some(HashTag::SighashCombinator))
    }
}
