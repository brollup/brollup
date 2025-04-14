use super::element_type::CallElementType;
use crate::constructive::entity::account::Account;
use crate::constructive::entity::contract::Contract;
use crate::constructive::valtype::long_val::LongVal;
use crate::constructive::valtype::maybe_common::maybe_common::MaybeCommon;
use crate::constructive::valtype::short_val::ShortVal;
use crate::inscriptive::registery::account_registery::ACCOUNT_REGISTERY;
use crate::inscriptive::registery::contract_registery::CONTRACT_REGISTERY;
use crate::inscriptive::registery::registery::REGISTERY;
use crate::{
    constructive::cpe::{
        cpe::CompactPayloadEncoding,
        decode_error::{
            calldata_error::{BytesDecodingError, CalldataCPEDecodingError, VarbytesDecodingError},
            error::CPEDecodingError,
        },
    },
    executive::stack::stack_item::StackItem,
};
use bit_vec::BitVec;

// Represents a single element of calldata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallElement {
    U8(u8),
    U16(u16),
    U32(ShortVal),
    U64(LongVal),
    Bool(bool),
    Account(Account),
    Contract(Contract),
    Bytes(Vec<u8>),
    Varbytes(Vec<u8>),
    Payable(ShortVal),
}

impl CallElement {
    /// Decodes a `CallElement` from a bit stream.
    pub async fn decode_cpe<'a>(
        bit_stream: &mut bit_vec::Iter<'_>,
        element_type: CallElementType,
        registery: &REGISTERY,
    ) -> Result<Self, CPEDecodingError> {
        // Match on the calldata element type.
        match element_type {
            // Decode the u8.
            CallElementType::U8 => {
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
                let element = CallElement::U8(value);

                // Return the element.
                Ok(element)
            }

            // Decode the u16.
            CallElementType::U16 => {
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
                let element = CallElement::U16(value);

                // Return the element.
                Ok(element)
            }

            // Decode the u32.
            CallElementType::U32 => {
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
                    .value();

                // Construct the `CalldataElement`.
                let element = CallElement::U32(short_val);

                // Return the element.
                Ok(element)
            }

            // Decode the u64.
            CallElementType::U64 => {
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
                    .value();

                let element = CallElement::U64(long_val);

                // Return the element.
                Ok(element)
            }

            // Decode the bool.
            CallElementType::Bool => {
                // Collect the bool value by iterating over a single bit.
                let bool = bit_stream
                    .next()
                    .ok_or(CPEDecodingError::CalldataCPEDecodingError(
                        CalldataCPEDecodingError::BoolDecodingError,
                    ))?;

                // Construct the `CalldataElement`.
                let element = CallElement::Bool(bool);

                // Return the element.
                Ok(element)
            }

            // Decode the `Account`.
            CallElementType::Account => {
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

                // Construct the `CalldataElement`.
                let element = CallElement::Account(account);

                // Return the element.
                Ok(element)
            }

            // Decode the `Contract`.
            CallElementType::Contract => {
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

                // Construct the `CallElement`.
                let element = CallElement::Contract(contract);

                // Return the element.
                Ok(element)
            }

            // Decode the `Bytes1-256`.
            CallElementType::Bytes(index) => {
                // Byte length is the index + 1.
                let byte_length = index as usize + 1;

                // Check if the data length is valid.
                if byte_length < 1 || byte_length > 256 {
                    return Err(CPEDecodingError::CalldataCPEDecodingError(
                        CalldataCPEDecodingError::BytesDecodingError(
                            BytesDecodingError::InvalidBytesLength,
                        ),
                    ));
                }

                // Get the number of bits to collect.
                let bit_length = byte_length as usize * 8;

                // Collect the data bits.
                let mut data_bits = BitVec::new();
                for _ in 0..bit_length {
                    data_bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::BytesDecodingError(
                                BytesDecodingError::UnableToCollectBytesDataBits,
                            ),
                        ),
                    )?);
                }

                // Convert the bits to data bytes.
                let data_bytes = data_bits.to_bytes();

                // Construct the `CalldataElement`.
                let element = CallElement::Bytes(data_bytes);

                // Return the element.
                Ok(element)
            }

            // Decode the `Varbytes`.
            CallElementType::Varbytes => {
                // Initialize a bit vector to fill with byte length.
                let mut byte_length_bits = BitVec::new();

                // Collect 12 bits representing the byte length.
                // Supported byte-length range: 0 to 4095.
                for _ in 0..12 {
                    byte_length_bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::VarbytesDecodingError(
                                VarbytesDecodingError::UnableToCollectVarbytesLengthBits,
                            ),
                        ),
                    )?);
                }

                // Convert the byte length bits to a u16.
                let byte_length = convert_12_bits_to_u16(&byte_length_bits);

                // Return an error if the byte length is greater than 4095.
                if byte_length > 4095 {
                    return Err(CPEDecodingError::CalldataCPEDecodingError(
                        CalldataCPEDecodingError::VarbytesDecodingError(
                            VarbytesDecodingError::VarbytesLengthGreaterThan4095,
                        ),
                    ));
                }

                // If the data length is 0, return an empty `Varbytes`.
                if byte_length == 0 {
                    return Ok(CallElement::Varbytes(vec![]));
                }

                // Convert to bit length.
                let bit_length = byte_length as usize * 8;

                // Initialize bit vector to fill with data.
                let mut data_bits = BitVec::new();

                // Collect the data bit by bit.
                for _ in 0..bit_length {
                    data_bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::VarbytesDecodingError(
                                VarbytesDecodingError::UnableToCollectVarbytesDataBits,
                            ),
                        ),
                    )?);
                }

                // Convert the bits to bytes.
                let data_bytes = data_bits.to_bytes();

                // Construct `CalldataElement` from the bytes.
                let element = CallElement::Varbytes(data_bytes);

                // Return the element.
                Ok(element)
            }

            // Decode the `Payable`.
            CallElementType::Payable => {
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
                    .value();

                // Construct the `CalldataElement`.
                let element = CallElement::Payable(short_val);

                // Return the element.
                Ok(element)
            }
        }
    }

    /// Returns the type of the element.
    pub fn element_type(&self) -> CallElementType {
        match self {
            CallElement::U8(_) => CallElementType::U8,
            CallElement::U16(_) => CallElementType::U16,
            CallElement::U32(_) => CallElementType::U32,
            CallElement::U64(_) => CallElementType::U64,
            CallElement::Bool(_) => CallElementType::Bool,
            CallElement::Account(_) => CallElementType::Account,
            CallElement::Contract(_) => CallElementType::Contract,
            CallElement::Bytes(bytes) => {
                // Byte length is the inner value + 1. So we need to subtract 1 from the length.
                let index = bytes.len() as u8 - 1;
                // Return the element type.
                CallElementType::Bytes(index)
            }
            CallElement::Varbytes(_) => CallElementType::Varbytes,
            CallElement::Payable(_) => CallElementType::Payable,
        }
    }

    /// Returns the element in the pure bytes format to be pushed/used for stack operations.
    pub fn stack_item(&self) -> StackItem {
        match self {
            // 1 byte in stack.
            CallElement::U8(value) => StackItem::new(vec![*value]),
            // 2 bytes in stack.
            CallElement::U16(value) => StackItem::new(value.to_le_bytes().to_vec()),
            // 4 bytes in stack.
            CallElement::U32(value) => StackItem::new(value.value().to_le_bytes().to_vec()),
            // 8 bytes in stack.
            CallElement::U64(value) => StackItem::new(value.value().to_le_bytes().to_vec()),
            // 1 byte in stack  .
            CallElement::Bool(value) => match value {
                // True is a single byte of 0x01.
                true => StackItem::new(vec![0x01]),
                // False is an empty stack item.
                false => StackItem::new(vec![]),
            },
            // 32 bytes in stack.
            CallElement::Account(value) => StackItem::new(value.key().serialize_xonly().to_vec()),
            // 32 bytes in stack.
            CallElement::Contract(value) => StackItem::new(value.contract_id().to_vec()),
            // 1-256 bytes in stack.
            CallElement::Bytes(bytes) => StackItem::new(bytes.clone()),
            // 0-4096 bytes in stack.
            CallElement::Varbytes(bytes) => StackItem::new(bytes.clone()),
            // 4 bytes in stack.
            CallElement::Payable(value) => StackItem::new(value.value().to_le_bytes().to_vec()),
        }
    }
}

impl CompactPayloadEncoding for CallElement {
    fn encode_cpe(&self) -> Option<BitVec> {
        match self {
            CallElement::U8(u8_value) => {
                // Get the u8 value.
                let value = *u8_value;

                // Convert value to bytes.
                let byte: [u8; 1] = value.to_le_bytes();

                // Convert bytes to bits.
                let bits = BitVec::from_bytes(&byte);

                // Return the bits.
                Some(bits)
            }
            CallElement::U16(u16_value) => {
                // Get the u16 value.
                let value = *u16_value;

                // Convert value to bytes.
                let bytes: [u8; 2] = value.to_le_bytes();

                // Convert bytes to bits.
                let bits = BitVec::from_bytes(&bytes);

                // Return the bits.
                Some(bits)
            }
            CallElement::U32(short_val) => {
                // Encode the `ShortVal`.
                let bits = short_val.encode_cpe()?;

                // Return the bits.
                Some(bits)
            }
            CallElement::U64(long_val) => {
                // Encode the `LongVal`.
                let bits = long_val.encode_cpe()?;

                // Return the bits.
                Some(bits)
            }
            CallElement::Bool(value) => {
                // Get the bool value.
                let bool = *value;

                // Initialize bit vector.
                let mut bits = BitVec::new();

                // Push the bool value.
                bits.push(bool);

                // Return the bits.
                Some(bits)
            }
            CallElement::Account(account) => {
                // Encode the `Account`.
                let bits = account.encode_cpe()?;

                // Return the bits.
                Some(bits)
            }
            CallElement::Contract(contract) => {
                // Encode the `Contract`.
                let bits = contract.encode_cpe()?;

                // Return the bits.
                Some(bits)
            }
            CallElement::Bytes(bytes) => {
                // Encode the bytes.
                let bits = BitVec::from_bytes(bytes);

                // Return the bits.
                Some(bits)
            }
            CallElement::Varbytes(bytes) => {
                // Initialize bit vector to fill with length plus data.
                let mut bits = BitVec::new();

                // Get the byte length value.
                let byte_length = bytes.len() as u16;

                // Byte length as 2 bytes.
                let byte_length_bits = convert_u16_to_12_bits(byte_length);

                // Extend the bit vector with the byte length.
                bits.extend(byte_length_bits);

                // If data length is 0, return the bit vector with length-bits-only.
                // This is to avoid encoding empty data, as data can be empty.
                if byte_length == 0 {
                    return Some(bits);
                }

                // Get the data bits.
                let data_bits = BitVec::from_bytes(bytes);

                // Extend the bit vector with the data bits.
                bits.extend(data_bits);

                // Return the bits.
                Some(bits)
            }
            CallElement::Payable(short_val) => {
                // Encode the `ShortVal`.
                let bits = short_val.encode_cpe()?;

                // Return the bits.
                Some(bits)
            }
        }
    }
}

/// Converts a u16 to 12 bits.
fn convert_u16_to_12_bits(value: u16) -> BitVec {
    // Byte length as 2 bytes.
    let byte_length_bytes = value.to_le_bytes();

    // Initialize byte length bits.
    let mut byte_length_bits = BitVec::new();

    // Convert byte length to bits.
    for i in 0..12 {
        let byte_idx = i / 8;
        let bit_idx = i % 8;
        byte_length_bits.push((byte_length_bytes[byte_idx] >> bit_idx) & 1 == 1);
    }

    // Return the bits.
    byte_length_bits
}

/// Converts 12 bits to a u16.
fn convert_12_bits_to_u16(bits: &BitVec) -> u16 {
    // Initialize a u16 value.
    let mut byte_length = 0u16;

    // Iterate over 12 bits.
    for i in 0..12 {
        let bit = bits[i];
        if bit {
            byte_length |= 1 << i;
        }
    }

    // Return the u16 value.
    byte_length
}
