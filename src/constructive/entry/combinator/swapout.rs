use crate::{
    hash::{Hash, HashTag},
    schnorr::Sighash,
    valtype::account::Account,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Swapout {
    account: Account,
    amount: u32,
    taproot_key: [u8; 32],
}

impl Swapout {
    pub fn new(account: Account, amount: u32, taproot_key: [u8; 32]) -> Swapout {
        Swapout {
            account,
            amount,
            taproot_key,
        }
    }

    pub fn account(&self) -> Account {
        self.account
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn taproot_key(&self) -> [u8; 32] {
        self.taproot_key
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

impl Sighash for Swapout {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        preimage.extend(self.account.key().serialize_xonly());
        preimage.extend(self.amount.to_le_bytes());
        preimage.extend(self.taproot_key);

        preimage.hash(Some(HashTag::SighashCombinator))
    }
}
