use crate::valtype::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transfer {
    from: Account,
    to: Account,
    amount: u32,
}

impl Transfer {
    pub fn new(from: Account, to: Account, amount: u32) -> Transfer {
        Transfer { from, to, amount }
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
}
