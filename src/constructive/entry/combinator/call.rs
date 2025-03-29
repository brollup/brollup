use crate::constructive::{entity::account::Account, entity::contract::Contract};
use crate::transmutive::hash::Hash;
use crate::transmutive::{hash::HashTag, schnorr::Sighash};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Call {
    from: Account,
    contract: Contract,
    calldata: Vec<Vec<u8>>,
}

impl Call {
    pub fn new(from: Account, contract: Contract, calldata: Vec<Vec<u8>>) -> Call {
        Call {
            from,
            contract,
            calldata,
        }
    }

    pub fn from(&self) -> Account {
        self.from
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn calldata(&self) -> Vec<Vec<u8>> {
        self.calldata.clone()
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

impl Sighash for Call {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        preimage.extend(self.from.key().serialize_xonly());
        preimage.extend(self.contract.contract_id());

        for calldata in self.calldata.iter() {
            preimage.extend(calldata);
        }

        preimage.hash(Some(HashTag::SighashCombinator))
    }
}
