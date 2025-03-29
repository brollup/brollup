use crate::{constructive::entity::account::Account, transmutive::schnorr::Sighash};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claim;

impl Claim {
    pub fn new() -> Self {
        Self {}
    }

    // TODO: Implement this.
    pub fn validate_account(&self, _account: Account) -> bool {
        true
    }
}

impl Sighash for Claim {
    fn sighash(&self) -> [u8; 32] {
        [0xffu8; 32]
    }
}
