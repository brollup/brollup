use crate::constructive::entity::account::Account;
use crate::transmutive::schnorr::Sighash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Deploy;

impl Deploy {
    pub fn new() -> Self {
        Self {}
    }

    // TODO: Implement this.
    pub fn validate_account(&self, _account: Account) -> bool {
        true
    }
}

impl Sighash for Deploy {
    fn sighash(&self) -> [u8; 32] {
        [0xffu8; 32]
    }
}
