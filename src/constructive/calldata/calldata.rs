use crate::cpe::{CPEDecodingError, CalldataCPEDecodingError};
use crate::entity::account::Account;
use crate::entity::contract::Contract;
use crate::registery::account_registery::ACCOUNT_REGISTERY;
use crate::registery::contract_registery::CONTRACT_REGISTERY;
use crate::registery::registery::REGISTERY;
use crate::valtype::long_val::LongVal;
use crate::valtype::maybe_common::maybe_common::MaybeCommon;
use crate::valtype::short_val::ShortVal;
use bit_vec::BitVec;

/// Represents the type of a single element of calldata.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CalldataElementType {
    // Represents an unsigned 8-bit integer.
    U8,
    // Represents an unsigned 16-bit integer.
    U16,
    // Represents an unsigned 32-bit integer.
    U32,
    // Represents an unsigned 64-bit integer.
    U64,
    // Represents a boolean value.
    Bool,
    // Represents an `Account`.
    Account,
    // Represents a `Contract`.
    Contract,
    // Represents a byte array with a known length.
    // Inner value is the length index of the bytes.
    // Supported byte-length range: 1-256 bytes
    Bytes(u8),
    // Represents a byte array with varying length.
    // Supported byte-length range: 0-1023 bytes
    Varbytes,
}

// Represents a single element of calldata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalldataElement {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Bool(bool),
    Account([u8; 32]),
    Contract([u8; 32]),
    Bytes(Vec<u8>),
    Varbytes(Vec<u8>),
}

impl CalldataElement {
    pub async fn decode_cpe<'a>(
        bit_stream: &mut bit_vec::Iter<'_>,
        element_type: CalldataElementType,
        registery: &REGISTERY,
    ) -> Result<Self, CPEDecodingError> {
        match element_type {
            // Decode the u8.
            CalldataElementType::U8 => {
                // Create a new bit vector.
                let mut bits = BitVec::new();

                // Collect 8 bits.
                for _ in 0..8 {
                    bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::U8DecodingError,
                        ),
                    )?);
                }

                // Convert to byte.
                let byte: [u8; 1] = bits.to_bytes().try_into().map_err(|_| {
                    CPEDecodingError::CalldataCPEDecodingError(
                        CalldataCPEDecodingError::U8DecodingError,
                    )
                })?;

                // Convert byte to a u8.
                let value = byte[0];

                // Construct the `CalldataElement`.
                let element = CalldataElement::U8(value);

                // Return the element.
                Ok(element)
            }

            // Decode the u16.
            CalldataElementType::U16 => {
                // Create a new bit vector.
                let mut bits = BitVec::new();

                // Collect 16 bits.
                for _ in 0..16 {
                    bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::U16DecodingError,
                        ),
                    )?);
                }

                // Convert to bytes.
                let bytes: [u8; 2] = bits.to_bytes().try_into().map_err(|_| {
                    CPEDecodingError::CalldataCPEDecodingError(
                        CalldataCPEDecodingError::U16DecodingError,
                    )
                })?;

                // Convert the bytes to a u16.
                let value = u16::from_le_bytes(bytes);

                // Construct the `CalldataElement`.
                let element = CalldataElement::U16(value);

                // Return the element.
                Ok(element)
            }

            // Decode the u32.
            CalldataElementType::U32 => {
                // Decode the `ShortVal` from `MaybeCommon<ShortVal>`.
                let short_val = MaybeCommon::<ShortVal>::decode_cpe(bit_stream)
                    .map_err(|e| {
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::U32DecodingError(match e {
                                CPEDecodingError::MaybeCommonCPEDecodingError(err) => err,
                                _ => return CPEDecodingError::UnexpectedError,
                            }),
                        )
                    })?
                    .inner_val();

                // Get the u32 value.
                let value = short_val.value();

                // Construct the `CalldataElement`.
                let element = CalldataElement::U32(value);

                // Return the element.
                Ok(element)
            }

            // Decode the u64.
            CalldataElementType::U64 => {
                // Decode the `LongVal` from `MaybeCommon<LongVal>`.
                let long_val = MaybeCommon::<LongVal>::decode_cpe(bit_stream)
                    .map_err(|e| {
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::U64DecodingError(match e {
                                CPEDecodingError::MaybeCommonCPEDecodingError(err) => err,
                                _ => return CPEDecodingError::UnexpectedError,
                            }),
                        )
                    })?
                    .inner_val();

                // Get the u64 value.
                let value = long_val.value();

                // Construct the `CalldataElement`.
                let element = CalldataElement::U64(value);

                // Return the element.
                Ok(element)
            }

            // Decode the bool.
            CalldataElementType::Bool => {
                // Collect the bool value by iterating over a single bit.
                let bool = bit_stream
                    .next()
                    .ok_or(CPEDecodingError::CalldataCPEDecodingError(
                        CalldataCPEDecodingError::BoolDecodingError,
                    ))?;

                // Construct the `CalldataElement`.
                let element = CalldataElement::Bool(bool);

                // Return the element.
                Ok(element)
            }

            // Decode the `Account`.
            CalldataElementType::Account => {
                // Get the account registry.
                let account_registry: ACCOUNT_REGISTERY = {
                    let _registery = registery.lock().await;
                    _registery.account_registery()
                };

                // Decode the `Account`.
                let account = Account::decode_cpe(bit_stream, &account_registry)
                    .await
                    .map_err(|e| {
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::AccountDecodingError(match e {
                                CPEDecodingError::AccountCPEDecodingError(err) => err,
                                _ => return CPEDecodingError::UnexpectedError,
                            }),
                        )
                    })?;

                // Get the account key bytes.
                let account_key = account.key().serialize_xonly();

                // Construct the `CalldataElement`.
                let element = CalldataElement::Account(account_key);

                // Return the element.
                Ok(element)
            }

            // Decode the `Contract`.
            CalldataElementType::Contract => {
                // Get the contract registry.
                let contract_registry: CONTRACT_REGISTERY = {
                    let _registery = registery.lock().await;
                    _registery.contract_registery()
                };

                // Decode the `Contract`.
                let contract = Contract::decode_cpe(bit_stream, &contract_registry)
                    .await
                    .map_err(|e| {
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::ContractDecodingError(match e {
                                CPEDecodingError::ContractCPEDecodingError(err) => err,
                                _ => return CPEDecodingError::UnexpectedError,
                            }),
                        )
                    })?;

                // Get the contract ID.
                let contract_id = contract.contract_id();

                // Construct the `CalldataElement`.
                let element = CalldataElement::Contract(contract_id);

                // Return the element.
                Ok(element)
            }

            // Decode the `Bytes1-256`.
            CalldataElementType::Bytes(len_index) => {
                // Byte length is `len_index + 1`.
                let byte_length: u16 = len_index as u16 + 1;

                // Check if the byte length is valid.
                if byte_length < 1 || byte_length > 256 {
                    return Err(CPEDecodingError::CalldataCPEDecodingError(
                        CalldataCPEDecodingError::BytesDecodingError,
                    ));
                }

                // Get the number of bits to collect.
                let bit_length = byte_length as usize * 8;

                // Collect the bits.
                let mut bits = BitVec::new();
                for _ in 0..bit_length {
                    bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::BytesDecodingError,
                        ),
                    )?);
                }

                // Convert the bits to bytes.
                let bytes = bits.to_bytes();

                // Construct the `CalldataElement`.
                let element = CalldataElement::Bytes(bytes);

                // Return the element.
                Ok(element)
            }

            // Decode the `Varbytes`.
            CalldataElementType::Varbytes => {
                // Initialize bit vector.
                let mut bits = BitVec::new();

                // Collect 10 bits.
                for _ in 0..10 {
                    bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::VarbytesDecodingError,
                        ),
                    )?);
                }

                // Convert 10 bits to 2-byte.
                let bytes: [u8; 2] = bits.to_bytes().try_into().map_err(|_| {
                    CPEDecodingError::CalldataCPEDecodingError(
                        CalldataCPEDecodingError::VarbytesDecodingError,
                    )
                })?;

                // Get the byte length as u16.
                let byte_length = u16::from_le_bytes(bytes);

                // Convert to bit length.
                let bit_length = byte_length as usize * 8;

                // Initialize bit vector.
                let mut bits = BitVec::new();

                // Collect the bits.
                for _ in 0..bit_length {
                    bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::VarbytesDecodingError,
                        ),
                    )?);
                }

                // Convert the bits to bytes.
                let bytes = bits.to_bytes();

                // Construct `CalldataElement` from the bytes.
                let element = CalldataElement::Varbytes(bytes);

                // Return the element.
                Ok(element)
            }
        }
    }
}
