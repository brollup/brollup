use crate::{
    cpe::{CPEDecodingError, CompactPayloadEncoding},
    registery::contract_registery::CONTRACT_REGISTERY,
    valtype::short::ShortVal,
};
use async_trait::async_trait;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// Represents a contract; a program that can be executed on the system.
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct Contract {
    contract_id: [u8; 32],
    registery_index: ShortVal,
}

impl Contract {
    /// Creates a new contract.
    pub fn new(contract_id: [u8; 32], registery_index: u32) -> Contract {
        // Convert the registery index to a ShortVal.
        let registery_index = ShortVal::new(registery_index);

        Contract {
            contract_id,
            registery_index,
        }
    }

    /// Returns the contract id.
    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    /// Returns the registery index.
    pub fn registery_index(&self) -> u32 {
        self.registery_index.value()
    }

    /// Serializes the contract.
    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    /// Compact payload decoding for `Contract`.
    /// Decodes a `Contract` from a bit stream and returns it along with the remaining bit stream.
    pub async fn decode_cpe<'a>(
        bit_stream: bit_vec::Iter<'a>,
        contract_registery: &'a CONTRACT_REGISTERY,
    ) -> Result<(Contract, bit_vec::Iter<'a>), CPEDecodingError> {
        // Decode registery index.
        let (registery_index, bit_stream) = ShortVal::decode_cpe(bit_stream)?;

        // Retrieve the contract.
        let contract = {
            let _contract_registery = contract_registery.lock().await;
            _contract_registery
                .contract_by_index(registery_index.value())
                .ok_or(CPEDecodingError::RegisteryError)?
        };

        // Return the contract and the remaining bit stream.
        Ok((contract, bit_stream))
    }
}

/// Compact payload encoding for `Contract`.
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
