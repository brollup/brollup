use super::nonces::NSessionNonces;
use crate::valtype::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NSessionRequest {
    msg_sender: Account,
    nonces: NSessionNonces,
}

impl NSessionRequest {
    pub fn new(key: Account, nonces: &NSessionNonces) -> NSessionRequest {
        NSessionRequest {
            msg_sender: key,
            nonces: nonces.to_owned(),
        }
    }

    pub fn msg_sender(&self) -> Account {
        self.msg_sender
    }

    pub fn nonces(&self) -> NSessionNonces {
        self.nonces.clone()
    }
}
