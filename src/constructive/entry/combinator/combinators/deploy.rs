use crate::constructive::entity::account::account::Account;
use crate::transmutative::secp::authenticable::AuthSighash;
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

impl AuthSighash for Deploy {
    fn auth_sighash(&self) -> [u8; 32] {
        [0xffu8; 32]
    }
}
