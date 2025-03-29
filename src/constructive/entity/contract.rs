use crate::{
    constructive::cpe::{
        cpe::CompactPayloadEncoding,
        decode_error::{entity_error::ContractCPEDecodingError, error::CPEDecodingError},
    },
    constructive::valtype::short_val::ShortVal,
    inscriptive::registery::contract_registery::CONTRACT_REGISTERY,
};
use async_trait::async_trait;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// Represents a contract; a program that can be executed on Brollup.
#[derive(Clone, Copy, Serialize, Deserialize, Hash, Debug)]
pub struct Contract {
    contract_id: [u8; 32],
    registery_index: ShortVal,
    rank: Option<ShortVal>,
}

impl Contract {
    /// Creates a new contract.
    pub fn new(contract_id: [u8; 32], registery_index: u32, rank: Option<u32>) -> Contract {
        // Convert the registery index to a ShortVal.
        let registery_index = ShortVal::new(registery_index);

        // Convert the rank to a ShortVal.
        let rank = match rank {
            Some(rank) => Some(ShortVal::new(rank)),
            None => None,
        };

        Contract {
            contract_id,
            registery_index,
            rank,
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

    /// Returns the rank (if set).
    pub fn rank(&self) -> Option<u32> {
        self.rank.map(|rank| rank.value())
    }

    /// Sets the rank index.
    pub fn set_rank(&mut self, rank: Option<u32>) {
        self.rank = rank.map(|rank| ShortVal::new(rank));
    }

    /// Serializes the contract.
    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    /// Compact payload decoding for `Contract`.
    /// Decodes a `Contract` from a bit stream.
    pub async fn decode_cpe<'a>(
        bit_stream: &mut bit_vec::Iter<'a>,
        contract_registery: &CONTRACT_REGISTERY,
    ) -> Result<Contract, CPEDecodingError> {
        // Decode the rank value.
        let rank = ShortVal::decode_cpe(bit_stream)
            .map_err(|_| {
                CPEDecodingError::ContractCPEDecodingError(
                    ContractCPEDecodingError::FailedToDecodeRank,
                )
            })?
            .value();

        // Retrieve the contract given rank value.
        let contract = {
            let _contract_registery = contract_registery.lock().await;
            _contract_registery.contract_by_rank(rank).ok_or(
                CPEDecodingError::ContractCPEDecodingError(
                    ContractCPEDecodingError::FailedToLocateContractGivenRank(rank),
                ),
            )?
        };

        // Return the contract.
        return Ok(contract);
    }
}

impl PartialEq for Contract {
    fn eq(&self, other: &Self) -> bool {
        self.contract_id == other.contract_id
    }
}

impl Eq for Contract {}

/// Compact payload encoding for `Contract`.
#[async_trait]
impl CompactPayloadEncoding for Contract {
    fn encode_cpe(&self) -> Option<BitVec> {
        // Initialize the bitvec.
        let mut bits = BitVec::new();

        // Get rank. Returns None if the contract has no given rank.
        let rank = self.rank?;

        // Extend rank bits.
        bits.extend(rank.encode_cpe()?);

        Some(bits)
    }
}
