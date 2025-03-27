use crate::{
    cpe::{
        cpe::CompactPayloadEncoding,
        decode_error::{entity_error::ContractCPEDecodingError, error::CPEDecodingError},
    },
    registery::contract_registery::CONTRACT_REGISTERY,
    valtype::short_val::ShortVal,
};
use async_trait::async_trait;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// Represents a contract; a program that can be executed on Brollup.
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct Contract {
    contract_id: [u8; 32],
    registery_index: ShortVal,
    rank: Option<u8>,
}

impl Contract {
    /// Creates a new contract.
    pub fn new(contract_id: [u8; 32], registery_index: u32, rank: Option<u8>) -> Contract {
        // Convert the registery index to a ShortVal.
        let registery_index = ShortVal::new(registery_index);

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
    pub fn rank(&self) -> Option<u8> {
        self.rank
    }

    /// Sets the rank index.
    pub fn set_rank(&mut self, rank: Option<u8>) {
        self.rank = rank;
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
        // Iterate one bit to check if the contract is top-ranked.
        let is_ranked = bit_stream
            .next()
            .ok_or(CPEDecodingError::ContractCPEDecodingError(
                ContractCPEDecodingError::FailedToCollectIsRankedBit,
            ))?;

        // Check if the contract is top-ranked.
        match is_ranked {
            true => {
                // Contract is one of the top 64 ranked contracts.

                // Initialize a bit vector to fill with rank index bits.
                let mut rank_index_bits = BitVec::new();

                // Collect 6 bits from the bit stream.
                for _ in 0..6 {
                    let bit =
                        bit_stream
                            .next()
                            .ok_or(CPEDecodingError::ContractCPEDecodingError(
                                ContractCPEDecodingError::FailedToCollectRankIndexBits,
                            ))?;
                    rank_index_bits.push(bit);
                }

                // Decode the rank index from the bit stream.
                let mut decoded_rank_index = 0u8;
                for i in 0..6 {
                    let bit = rank_index_bits[i];
                    if bit {
                        decoded_rank_index |= 1 << i;
                    }
                }

                // Rank is the rank index + 1.
                let rank = decoded_rank_index + 1;

                // Retrieve the contract given rank index.
                let contract = {
                    let _contract_registery = contract_registery.lock().await;
                    _contract_registery.contract_by_rank(rank).ok_or(
                        CPEDecodingError::ContractCPEDecodingError(
                            ContractCPEDecodingError::FailedToLocateContractGivenRankIndex(
                                decoded_rank_index,
                            ),
                        ),
                    )?
                };

                // Return the contract.
                return Ok(contract);
            }
            false => {
                // Contract is not one of the top 64 ranked contracts.
                // Decode registery index from the bit stream.
                let registery_index = ShortVal::decode_cpe(bit_stream).map_err(|_| {
                    CPEDecodingError::ContractCPEDecodingError(
                        ContractCPEDecodingError::FailedToDecodeRegisteryIndex,
                    )
                })?;

                // Retrieve the contract given registery index.
                let contract = {
                    let _contract_registery = contract_registery.lock().await;
                    _contract_registery
                        .contract_by_registery_index(registery_index.value())
                        .ok_or(CPEDecodingError::ContractCPEDecodingError(
                            ContractCPEDecodingError::FailedToLocateContractGivenRegisteryIndex(
                                registery_index.value(),
                            ),
                        ))?
                };

                // Return the contract.
                return Ok(contract);
            }
        }
    }
}

/// Compact payload encoding for `Contract`.
#[async_trait]
impl CompactPayloadEncoding for Contract {
    fn encode_cpe(&self) -> BitVec {
        // Initialize the bitvec.
        let mut bits = BitVec::new();

        // Match if the contract is top-ranked.
        match self.rank {
            Some(rank) => {
                // Contract is one of the top 64 ranked contracts.

                // Push true for ranked.
                bits.push(true);

                // Rank index is the rank - 1.
                // This is because rank starts with #1.
                let rank_index = rank - 1;

                // Convert the rank index (u8) into a byte.
                let rank_index_bytes = rank_index.to_le_bytes();

                // Initialize the rank index bits vector.
                let mut rank_index_bits = BitVec::new();

                // Convert the rank index (u8) into a BitVec.
                for i in 0..6 {
                    rank_index_bits.push((rank_index_bytes[0] >> i) & 1 == 1);
                }

                // Extend the rank index bits.
                bits.extend(rank_index_bits);
            }
            None => {
                // Push false for unranked.
                bits.push(false);

                // Decode the registery index into a bitvec.
                let registery_index_bits = self.registery_index.encode_cpe();

                // Extend registery index bits.
                bits.extend(registery_index_bits);
            }
        }

        bits
    }
}
