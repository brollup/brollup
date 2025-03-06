use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Outpoint {
    prev: [u8; 32],
    vout: u32,
}

impl Outpoint {
    pub fn new(prev: [u8; 32], vout: u32) -> Outpoint {
        Outpoint { prev, vout }
    }
    pub fn prev(&self) -> [u8; 32] {
        self.prev
    }

    pub fn vout(&self) -> u32 {
        self.vout
    }

    pub fn vout_bytes(&self) -> [u8; 4] {
        self.vout.to_be_bytes()
    }

    pub fn bytes(&self) -> [u8; 36] {
        let mut bytes: [u8; 36] = [0; 36];
        bytes[..32].copy_from_slice(&self.prev);
        bytes[32..36].copy_from_slice(&self.vout_bytes());
        bytes
    }
}
