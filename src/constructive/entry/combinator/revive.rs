use crate::{constructive::entity::account::Account, transmutive::schnorr::Sighash};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Revive;

impl Revive {
    pub fn new() -> Self {
        Self {}
    }

    /// Validate the account.
    pub fn validate_account(&self, _account: Account) -> bool {
        true
    }
}

impl Sighash for Revive {
    fn sighash(&self) -> [u8; 32] {
        [0xffu8; 32]
    }
}
