use crate::cpe::{CPEDecodingError, CalldataCPEDecodingError, CompactPayloadEncoding};
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
    // Inner value is the byte length.
    // Supported byte-length range: 1-256 bytes
    Bytes(u16),
    // Represents a byte array with varying length.
    // Supported byte-length range: 0-4096 bytes
    Varbytes,
}

impl CalldataElementType {
    /// Returns the contract bytecode representation of the element type.
    pub fn contract_bytecode(&self) -> [u8; 3] {
        match self {
            CalldataElementType::U8 => [0x01, 0x00, 0x00],
            CalldataElementType::U16 => [0x02, 0x00, 0x00],
            CalldataElementType::U32 => [0x03, 0x00, 0x00],
            CalldataElementType::U64 => [0x04, 0x00, 0x00],
            CalldataElementType::Bool => [0x05, 0x00, 0x00],
            CalldataElementType::Account => [0x06, 0x00, 0x00],
            CalldataElementType::Contract => [0x07, 0x00, 0x00],
            CalldataElementType::Bytes(index) => {
                // Convert index to bytes.
                let bytes = index.to_le_bytes();

                // Return the bytes.
                [0x08, bytes[0], bytes[1]]
            }
            CalldataElementType::Varbytes => [0x09, 0x00, 0x00],
        }
    }

    /// Returns the element type from the contract bytecode.
    pub fn from_contract_bytecode(bytecode: [u8; 3]) -> Option<Self> {
        match bytecode {
            [0x01, 0x00, 0x00] => Some(CalldataElementType::U8),
            [0x02, 0x00, 0x00] => Some(CalldataElementType::U16),
            [0x03, 0x00, 0x00] => Some(CalldataElementType::U32),
            [0x04, 0x00, 0x00] => Some(CalldataElementType::U64),
            [0x05, 0x00, 0x00] => Some(CalldataElementType::Bool),
            [0x06, 0x00, 0x00] => Some(CalldataElementType::Account),
            [0x07, 0x00, 0x00] => Some(CalldataElementType::Contract),
            [0x08, byte_1, byte_2] => {
                let length_bytes = [byte_1, byte_2];
                let length = u16::from_le_bytes(length_bytes);
                Some(CalldataElementType::Bytes(length))
            }
            [0x09, 0x00, 0x00] => Some(CalldataElementType::Varbytes),
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
                    .inner_val();

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
                    .inner_val();

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
            CalldataElementType::Bytes(byte_length) => {
                // Check if the data length is valid.
                if byte_length < 1 || byte_length > 256 {
                    return Err(CPEDecodingError::CalldataCPEDecodingError(
                        CalldataCPEDecodingError::BytesDecodingError,
                    ));
                }

                // Get the number of bits to collect.
                let bit_length = byte_length as usize * 8;

                // Collect the data bits.
                let mut data_bits = BitVec::new();
                for _ in 0..bit_length {
                    data_bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::BytesDecodingError,
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

                // Collect 16 bits representing the byte length.
                for _ in 0..16 {
                    byte_length_bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::VarbytesDecodingError,
                        ),
                    )?);
                }

                // Convert to 2-bytes.
                let byte_length_bytes: [u8; 2] =
                    byte_length_bits.to_bytes().try_into().map_err(|_| {
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::VarbytesDecodingError,
                        )
                    })?;

                // Get the length value as u16.
                let byte_length = u16::from_le_bytes(byte_length_bytes);

                // Convert to bit length.
                let bit_length = byte_length as usize * 8;

                // Initialize bit vector to fill with data.
                let mut data_bits = BitVec::new();

                // Collect the data bit by bit.
                for _ in 0..bit_length {
                    data_bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::CalldataCPEDecodingError(
                            CalldataCPEDecodingError::VarbytesDecodingError,
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
            CalldataElement::Bytes(bytes) => CalldataElementType::Bytes(bytes.len() as u16),
            CalldataElement::Varbytes(_) => CalldataElementType::Varbytes,
        }
    }

    /// Returns the element in the pure bytes format to be pushed/used for stack operations.
    pub fn stack_bytes(&self) -> Vec<u8> {
        match self {
            // 1 byte in stack.
            CalldataElement::U8(value) => vec![*value],
            // 2 bytes in stack.
            CalldataElement::U16(value) => value.to_le_bytes().to_vec(),
            // 4 bytes in stack.
            CalldataElement::U32(value) => value.value().to_le_bytes().to_vec(),
            // 8 bytes in stack.
            CalldataElement::U64(value) => value.value().to_le_bytes().to_vec(),
            // 1 byte in stack  .
            CalldataElement::Bool(value) => vec![*value as u8],
            // 32 bytes in stack.
            CalldataElement::Account(value) => value.key().serialize_xonly().to_vec(),
            // 32 bytes in stack.
            CalldataElement::Contract(value) => value.contract_id().to_vec(),
            // 1-256 bytes in stack.
            CalldataElement::Bytes(bytes) => bytes.clone(),
            // 0-4096 bytes in stack.
            CalldataElement::Varbytes(bytes) => bytes.clone(),
        }
    }
}

impl CompactPayloadEncoding for CalldataElement {
    fn encode_cpe(&self) -> BitVec {
        match self {
            CalldataElement::U8(u8_value) => {
                // Get the u8 value.
                let value = *u8_value;

                // Convert value to bytes.
                let byte: [u8; 1] = value.to_le_bytes();

                // Convert bytes to bits.
                let bits = BitVec::from_bytes(&byte);

                // Return the bits.
                bits
            }
            CalldataElement::U16(u16_value) => {
                // Get the u16 value.
                let value = *u16_value;

                // Convert value to bytes.
                let bytes: [u8; 2] = value.to_le_bytes();

                // Convert bytes to bits.
                let bits = BitVec::from_bytes(&bytes);

                // Return the bits.
                bits
            }
            CalldataElement::U32(short_val) => {
                // Encode the `ShortVal`.
                let bits = short_val.encode_cpe();

                // Return the bits.
                bits
            }
            CalldataElement::U64(long_val) => {
                // Encode the `LongVal`.
                let bits = long_val.encode_cpe();

                // Return the bits.
                bits
            }
            CalldataElement::Bool(value) => {
                // Get the bool value.
                let bool = *value;

                // Initialize bit vector.
                let mut bits = BitVec::new();

                // Push the bool value.
                bits.push(bool);

                // Return the bits.
                bits
            }
            CalldataElement::Account(account) => {
                // Encode the `Account`.
                let bits = account.encode_cpe();

                // Return the bits.
                bits
            }
            CalldataElement::Contract(contract) => {
                // Encode the `Contract`.
                let bits = contract.encode_cpe();

                // Return the bits.
                bits
            }
            CalldataElement::Bytes(bytes) => {
                // Encode the bytes.
                let bits = BitVec::from_bytes(bytes);

                // Return the bits.
                bits
            }
            CalldataElement::Varbytes(bytes) => {
                // Initialize bit vector to fill with length plus data.
                let mut bits = BitVec::new();

                // Get the byte length value.
                let byte_length = bytes.len() as u16;

                // Byte length as 2 bytes.
                let byte_length_bytes: [u8; 2] = byte_length.to_le_bytes();

                // Convert byte length to bits.
                let byte_length_bits = BitVec::from_bytes(&byte_length_bytes);

                // Extend the bit vector with the byte length.
                bits.extend(byte_length_bits);

                // Get the data bits.
                let data_bits = BitVec::from_bytes(bytes);

                // Extend the bit vector with the data bits.
                bits.extend(data_bits);

                // Return the bits.
                bits
            }
        }
    }
}
