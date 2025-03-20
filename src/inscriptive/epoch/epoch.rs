use crate::entity::account::Account;
use secp::Point;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Epoch {
    height: u64,
    active: bool,
    group_key: Point,
    operators: Vec<Account>,
}

impl Epoch {
    pub fn new(height: u64, active: bool, group_key: Point, operators: Vec<Account>) -> Epoch {
        Epoch {
            height,
            active,
            group_key,
            operators,
        }
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn group_key(&self) -> Point {
        self.group_key.clone()
    }

    pub fn operators(&self) -> Vec<Account> {
        self.operators.clone()
    }

    pub fn operator_keys(&self) -> Vec<Point> {
        self.operators
            .iter()
            .map(|operator| operator.key())
            .collect()
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }
}
