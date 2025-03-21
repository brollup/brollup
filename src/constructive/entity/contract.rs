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
}

#[async_trait]
impl CompactPayloadEncoding for Contract {
    fn encode(&self) -> BitVec {
        // Initialize the bitvec.
        let mut bits = BitVec::new();

        // Registery index bits.
        let registery_index_bits = self.registery_index.encode();

        // Extend registery index bits.
        bits.extend(registery_index_bits);

        bits
    }

    async fn decode(
        bits: &BitVec,
        registery: Option<&REGISTERY>,
    ) -> Result<(Contract, BitVec), CPEError> {
        // Decode registery index.
        let (registery_index, remaining_bits) = ShortVal::decode(&bits, None).await?;

        // Get the contract registery.
        let contract_registery: CONTRACT_REGISTERY = match registery {
            Some(registery) => {
                let _registery = registery.lock().await;
                _registery.contract_registery()
            }
            None => return Err(CPEError::RegisteryError),
        };

        // Construct the contract.
        let contract = {
            let _contract_registery = contract_registery.lock().await;
            _contract_registery
                .contract_by_index(registery_index.value())
                .ok_or(CPEError::RegisteryError)?
        };

        Ok((contract, remaining_bits))
    }
}
