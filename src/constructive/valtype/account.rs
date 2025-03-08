use secp::Point;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Account {
    key: Point,
    registery_index: Option<u32>,
}

impl Account {
    pub fn new(key: Point, registery_index: Option<u32>) -> Option<Account> {
        let is_odd: bool = key.parity().into();

        if is_odd {
            return None;
        }

        let account = Account {
            key,
            registery_index,
        };

        Some(account)
    }

    pub fn set_registery_index(&mut self, registery_index: u32) {
        self.registery_index = Some(registery_index);
    }

    pub fn key(&self) -> Point {
        self.key
    }

    pub fn registery_index(&self) -> Option<u32> {
        self.registery_index
    }

    pub fn is_odd_key(&self) -> bool {
        self.key.parity().into()
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Account {}