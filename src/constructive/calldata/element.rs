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
    // Byte length is the inner value + 1.
    // Supported byte-length range: 1-256 bytes
    Bytes(u8),
    // Represents a byte array with an unknown length.
    // Supported byte-length range: 0-4096 bytes
    Varbytes,
}

impl CalldataElementType {
    /// Returns the contract bytecode representation of the element type.
    pub fn bytecode(&self) -> [u8; 2] {
        match self {
            CalldataElementType::U8 => [0x00, 0x00],
            CalldataElementType::U16 => [0x01, 0x00],
            CalldataElementType::U32 => [0x02, 0x00],
            CalldataElementType::U64 => [0x03, 0x00],
            CalldataElementType::Bool => [0x04, 0x00],
            CalldataElementType::Account => [0x05, 0x00],
            CalldataElementType::Contract => [0x06, 0x00],
            CalldataElementType::Bytes(index) => {
                // Return the bytes.
                [0x07, index.to_owned()]
            }
            CalldataElementType::Varbytes => [0x08, 0x00],
        }
    }

    /// Returns the element type from the contract bytecode.
    pub fn from_bytecode(bytecode: [u8; 2]) -> Option<Self> {
        match bytecode {
            [0x00, 0x00] => Some(CalldataElementType::U8),
            [0x01, 0x00] => Some(CalldataElementType::U16),
            [0x02, 0x00] => Some(CalldataElementType::U32),
            [0x03, 0x00] => Some(CalldataElementType::U64),
            [0x04, 0x00] => Some(CalldataElementType::Bool),
            [0x05, 0x00] => Some(CalldataElementType::Account),
            [0x06, 0x00] => Some(CalldataElementType::Contract),
            [0x07, index] => Some(CalldataElementType::Bytes(index)),
            [0x08, 0x00] => Some(CalldataElementType::Varbytes),
            _ => None,
        }
    }
}

// Represents a single element of calldata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalldataElement {
    U8(u8),
    U16(u16),
    U32(ShortVal),
    U64(LongVal),
    Bool(bool),
    Account(Account),
    Contract(Contract),
    Bytes(Vec<u8>),
    Varbytes(Vec<u8>),
}

impl CalldataElement {
    /// Decodes a `CalldataElement` from a bit stream.
    pub async fn decode_cpe<'a>(
        bit_stream: &mut bit_vec::Iter<'_>,
        element_type: CalldataElementType,
        registery: &REGISTERY,
    ) -> Result<Self, CPEDecodingError> {
        // Match on the calldata element type.
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
                    .value();

                // Construct the `CalldataElement`.
                let element = CalldataElement::U32(short_val);

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
                    .value();

                let element = CalldataElement::U64(long_val);

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

                // Construct the `CalldataElement`.
                let element = CalldataElement::Account(account);

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

                // Construct the `CalldataElement`.
                let element = CalldataElement::Contract(contract);

                // Return the element.
                Ok(element)
            }

            // Decode the `Bytes1-256`.
            CalldataElementType::Bytes(index) => {
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
                let element = CalldataElement::Bytes(data_bytes);

                // Return the element.
                Ok(element)
            }

            // Decode the `Varbytes`.
            CalldataElementType::Varbytes => {
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
                    return Ok(CalldataElement::Varbytes(vec![]));
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
                let element = CalldataElement::Varbytes(data_bytes);

                // Return the element.
                Ok(element)
            }
        }
    }

    /// Returns the type of the element.
    pub fn element_type(&self) -> CalldataElementType {
        match self {
            CalldataElement::U8(_) => CalldataElementType::U8,
            CalldataElement::U16(_) => CalldataElementType::U16,
            CalldataElement::U32(_) => CalldataElementType::U32,
            CalldataElement::U64(_) => CalldataElementType::U64,
            CalldataElement::Bool(_) => CalldataElementType::Bool,
            CalldataElement::Account(_) => CalldataElementType::Account,
            CalldataElement::Contract(_) => CalldataElementType::Contract,
            CalldataElement::Bytes(bytes) => {
                // Byte length is the inner value + 1. So we need to subtract 1 from the length.
                let index = bytes.len() as u8 - 1;
                // Return the element type.
                CalldataElementType::Bytes(index)
            }
            CalldataElement::Varbytes(_) => CalldataElementType::Varbytes,
        }
    }

    /// Returns the element in the pure bytes format to be pushed/used for stack operations.
    pub fn stack_item(&self) -> StackItem {
        match self {
            // 1 byte in stack.
            CalldataElement::U8(value) => StackItem::new(vec![*value]),
            // 2 bytes in stack.
            CalldataElement::U16(value) => StackItem::new(value.to_le_bytes().to_vec()),
            // 4 bytes in stack.
            CalldataElement::U32(value) => StackItem::new(value.value().to_le_bytes().to_vec()),
            // 8 bytes in stack.
            CalldataElement::U64(value) => StackItem::new(value.value().to_le_bytes().to_vec()),
            // 1 byte in stack  .
            CalldataElement::Bool(value) => match value {
                // True is a single byte of 0x01.
                true => StackItem::new(vec![0x01]),
                // False is an empty stack item.
                false => StackItem::new(vec![]),
            },
            // 32 bytes in stack.
            CalldataElement::Account(value) => {
                StackItem::new(value.key().serialize_xonly().to_vec())
            }
            // 32 bytes in stack.
            CalldataElement::Contract(value) => StackItem::new(value.contract_id().to_vec()),
            // 1-256 bytes in stack.
            CalldataElement::Bytes(bytes) => StackItem::new(bytes.clone()),
            // 0-4096 bytes in stack.
            CalldataElement::Varbytes(bytes) => StackItem::new(bytes.clone()),
        }
    }
}

impl CompactPayloadEncoding for CalldataElement {
    fn encode_cpe(&self) -> Option<BitVec> {
        match self {
            CalldataElement::U8(u8_value) => {
                // Get the u8 value.
                let value = *u8_value;

                // Convert value to bytes.
                let byte: [u8; 1] = value.to_le_bytes();

                // Convert bytes to bits.
                let bits = BitVec::from_bytes(&byte);

                // Return the bits.
                Some(bits)
            }
            CalldataElement::U16(u16_value) => {
                // Get the u16 value.
                let value = *u16_value;

                // Convert value to bytes.
                let bytes: [u8; 2] = value.to_le_bytes();

                // Convert bytes to bits.
                let bits = BitVec::from_bytes(&bytes);

                // Return the bits.
                Some(bits)
            }
            CalldataElement::U32(short_val) => {
                // Encode the `ShortVal`.
                let bits = short_val.encode_cpe()?;

                // Return the bits.
                Some(bits)
            }
            CalldataElement::U64(long_val) => {
                // Encode the `LongVal`.
                let bits = long_val.encode_cpe()?;

                // Return the bits.
                Some(bits)
            }
            CalldataElement::Bool(value) => {
                // Get the bool value.
                let bool = *value;

                // Initialize bit vector.
                let mut bits = BitVec::new();

                // Push the bool value.
                bits.push(bool);

                // Return the bits.
                Some(bits)
            }
            CalldataElement::Account(account) => {
                // Encode the `Account`.
                let bits = account.encode_cpe()?;

                // Return the bits.
                Some(bits)
            }
            CalldataElement::Contract(contract) => {
                // Encode the `Contract`.
                let bits = contract.encode_cpe()?;

                // Return the bits.
                Some(bits)
            }
            CalldataElement::Bytes(bytes) => {
                // Encode the bytes.
                let bits = BitVec::from_bytes(bytes);

                // Return the bits.
                Some(bits)
            }
            CalldataElement::Varbytes(bytes) => {
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
