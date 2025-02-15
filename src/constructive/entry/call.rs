use crate::valtype::{account::Account, contract::Contract};
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
}
