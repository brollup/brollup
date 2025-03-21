use crate::{
    cpe::{CPEError, CompactPayloadEncoding},
    registery::{contract_registery::CONTRACT_REGISTERY, registery::REGISTERY},
    valtype::short::ShortVal,
};
use async_trait::async_trait;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct Contract {
    contract_id: [u8; 32],
    registery_index: ShortVal,
}

impl Contract {
    pub fn new(contract_id: [u8; 32], registery_index: u32) -> Contract {
        // Convert the registery index to a ShortVal.
        let registery_index = ShortVal::new(registery_index);

        Contract {
            contract_id,
            registery_index,
        }
    }

    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    pub fn registery_index(&self) -> u32 {
        self.registery_index.value()
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub async fn decode_cpe(
        bit_stream: bit_vec::Iter<'_>,
        registery: REGISTERY,
    ) -> Result<(Contract, bit_vec::Iter<'_>), CPEError> {
        // Decode registery index.
        let (registery_index, bit_stream) = ShortVal::decode_cpe(bit_stream)?;

        // Get the contract registery.
        let contract_registery: CONTRACT_REGISTERY = {
            let _registery = registery.lock().await;
            _registery.contract_registery()
        };

        // Construct the contract.
        let contract = {
            let _contract_registery = contract_registery.lock().await;
            _contract_registery
                .contract_by_index(registery_index.value())
                .ok_or(CPEError::RegisteryError)?
        };

        Ok((contract, bit_stream))
    }
}

#[async_trait]
impl CompactPayloadEncoding for Contract {
    fn encode_cpe(&self) -> BitVec {
        // Initialize the bitvec.
        let mut bits = BitVec::new();

        // Registery index bits.
        let registery_index_bits = self.registery_index.encode_cpe();

        // Extend registery index bits.
        bits.extend(registery_index_bits);

        bits
    }
}
