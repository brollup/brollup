use crate::constructive::entity::account::Account;
use serde::{Deserialize, Serialize};

/// Liquidity provider
#[derive(Clone, Serialize, Deserialize)]
pub struct LP {
    account: Account,
    liquidity: u64,
    lp: u64,
}

impl LP {
    pub fn new(account: Account, liquidity: u64, lp: u64) -> LP {
        LP {
            account,
            liquidity,
            lp,
        }
    }

    pub fn account(&self) -> Account {
        self.account
    }

    pub fn liquidity(&self) -> u64 {
        self.liquidity
    }

    pub fn lp(&self) -> u64 {
        self.lp
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn update_add(&mut self, liquidity_add: u64, lp_add: u64) -> bool {
        let new_liquidity = match self.liquidity.checked_add(liquidity_add) {
            Some(num) => num,
            None => return false,
        };

        let new_lp = match self.lp.checked_add(lp_add) {
            Some(num) => num,
            None => return false,
        };

        self.liquidity = new_liquidity;
        self.lp = new_lp;

        true
    }

    pub fn update_remove(&mut self, liquidity_remove: u64, lp_remove: u64) -> bool {
        let new_liquidity = match self.liquidity.checked_sub(liquidity_remove) {
            Some(num) => num,
            None => return false,
        };

        let new_lp = match self.lp.checked_sub(lp_remove) {
            Some(num) => num,
            None => return false,
        };

        self.liquidity = new_liquidity;
        self.lp = new_lp;

        true
    }

    pub fn empty_liquidity(&self) -> bool {
        self.lp == 0
    }
}
