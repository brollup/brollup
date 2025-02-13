use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Contract {
    contract_id: [u8; 32],
    registery_index: u32,
}

impl Contract {
    pub fn new(contract_id: [u8; 32], registery_index: u32) -> Contract {
        Contract {
            contract_id,
            registery_index,
        }
    }

    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    pub fn registery_index(&self) -> u32 {
        self.registery_index
    }
}
