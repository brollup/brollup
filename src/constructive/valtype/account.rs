use secp::Point;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Account {
    key: Point,
    registery_index: Option<u32>,
}

impl Account {
    pub fn new(key: Point, registery_index: Option<u32>) -> Account {
        Account {
            key,
            registery_index,
        }
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
}
