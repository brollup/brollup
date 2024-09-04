#![allow(dead_code)]

use super::value::ShortVal;
use crate::encoding::cpe::CompactPayloadEncoding;
use bit_vec::BitVec;

#[derive(Clone, Copy)]
pub struct Contract {
    contract_id: [u8; 32],
    contract_index: Option<u32>,
}

impl Contract {
    pub fn new(contract_id: [u8; 32]) -> Contract {
        Contract {
            contract_id,
            contract_index: None,
        }
    }

    pub fn new_compact(contract_id: [u8; 32], contract_index: u32) -> Contract {
        Contract {
            contract_id,
            contract_index: Some(contract_index),
        }
    }

    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    pub fn contract_index(&self) -> Option<u32> {
        self.contract_index
    }

    pub fn set_contract_index(&mut self, contract_index: u32) {
        self.contract_index = Some(contract_index);
    }
}

impl CompactPayloadEncoding for Contract {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        match self.contract_index {
            None => {
                // Non-compact form
                bit_vec.push(false);

                let id_array = self.contract_id;
                let id_bits = BitVec::from_bytes(&id_array);

                bit_vec.extend(id_bits);
            }
            Some(index) => {
                // Compact form
                bit_vec.push(true);

                // ShortAmount represents compact integer forms
                let index_compact = ShortVal(index);

                bit_vec.extend(index_compact.to_cpe());
            }
        }

        bit_vec
    }
}
