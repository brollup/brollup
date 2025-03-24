use crate::cpe::{CPEDecodingError, CompactPayloadEncoding, ShortValCPEDecodingError};
use async_trait::async_trait;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

use super::common::CommonInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShortVal(pub u32);

/// Represents the tier of an uncommon `ShortVal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UncommonShortValTier {
    U8,
    U16,
    U24,
    U32,
}

impl ShortVal {
    /// Creates a new `ShortVal`
    pub fn new(short_val: u32) -> Self {
        Self(short_val)
    }

    /// Returns the core u32 value.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Determines the tier based on the value range.
    pub fn uncommon_tier(&self) -> UncommonShortValTier {
        match self.value() {
            0..=255 => UncommonShortValTier::U8,
            256..=65535 => UncommonShortValTier::U16,
            65536..=16777215 => UncommonShortValTier::U24,
            16777216..=4294967295 => UncommonShortValTier::U32,
        }
    }

    /// Returns the compact byte representation of the value if it is uncommon.
    pub fn compact_bytes(&self) -> Vec<u8> {
        let value = self.value();

        match self.uncommon_tier() {
            // 1 byte
            UncommonShortValTier::U8 => vec![value as u8],
            // 2 bytes
            UncommonShortValTier::U16 => {
                let bytes = value.to_le_bytes();
                vec![bytes[0], bytes[1]]
            }
            // 3 bytes
            UncommonShortValTier::U24 => {
                let bytes = value.to_le_bytes();
                vec![bytes[0], bytes[1], bytes[2]]
            }
            // 4 bytes
            UncommonShortValTier::U32 => value.to_le_bytes().to_vec(),
        }
    }

    /// Constructs a `ShortVal` from its compact byte representation.
    pub fn from_compact_bytes(bytes: &[u8]) -> Option<Self> {
        let value = match bytes.len() {
            // 1 byte (u8)
            1 => bytes[0] as u32,
            // 2 bytes (u16)
            2 => u16::from_le_bytes([bytes[0], bytes[1]]) as u32,
            // 3 bytes (u24)
            3 => {
                let mut buf = [0u8; 4];
                buf[..3].copy_from_slice(bytes);
                u32::from_le_bytes(buf)
            }
            // 4 bytes (u32)
            4 => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            _ => return None,
        };

        Some(Self(value))
    }

    /// Compact payload decoding for `ShortVal`.
    /// Decodes a `ShortVal` from a bit stream.
    pub fn decode_cpe(bit_stream: &mut bit_vec::Iter<'_>) -> Result<ShortVal, CPEDecodingError> {
        // Check if the value is common.
        let is_common = bit_stream
            .next()
            .ok_or(CPEDecodingError::ShortValCPEDecodingError(
                ShortValCPEDecodingError::BitStreamIteratorError,
            ))?;

        // Match the value.
        match is_common {
            true => {
                // Decode the common integer.
                let common_val = CommonInt::decode_cpe(bit_stream).map_err(|_| {
                    CPEDecodingError::ShortValCPEDecodingError(
                        ShortValCPEDecodingError::CommonIntDecodingError,
                    )
                })?;

                // Return the `ShortVal`.
                Ok(ShortVal::new(common_val))
            }
            false => {
                // Decode the uncommon tier.
                let tier = match (bit_stream.next(), bit_stream.next()) {
                    (Some(false), Some(false)) => UncommonShortValTier::U8,
                    (Some(false), Some(true)) => UncommonShortValTier::U16,
                    (Some(true), Some(false)) => UncommonShortValTier::U24,
                    (Some(true), Some(true)) => UncommonShortValTier::U32,
                    _ => {
                        return Err(CPEDecodingError::ShortValCPEDecodingError(
                            ShortValCPEDecodingError::BitStreamIteratorError,
                        ))
                    }
                };

                // Get the bit count for the tier.
                let bit_count = match tier {
                    UncommonShortValTier::U8 => 8,
                    UncommonShortValTier::U16 => 16,
                    UncommonShortValTier::U24 => 24,
                    UncommonShortValTier::U32 => 32,
                };

                // Collect the value bits.
                let mut value_bits = BitVec::new();
                for _ in 0..bit_count {
                    value_bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::ShortValCPEDecodingError(
                            ShortValCPEDecodingError::BitStreamIteratorError,
                        ),
                    )?);
                }

                // Convert the value bits to bytes.
                let value_bytes = value_bits.to_bytes();

                // Construct the short value.
                let short_val = ShortVal::from_compact_bytes(&value_bytes).ok_or(
                    CPEDecodingError::ShortValCPEDecodingError(
                        ShortValCPEDecodingError::ShortValConversionError,
                    ),
                )?;

                // Return the `ShortVal`.
                Ok(short_val)
            }
        }
    }
}

#[async_trait]
impl CompactPayloadEncoding for ShortVal {
    fn encode_cpe(&self) -> BitVec {
        let mut bits = BitVec::new();

        // Try to encode self value as a common integer.
        match CommonInt::encode(self.value()) {
            Some(common_int_bits) => {
                // Push true for the common integer.
                bits.push(true);

                // Push the common integer bits.
                bits.extend(common_int_bits);

                // Return the bits.
                return bits;
            }
            None => {
                // Push false for the uncommon integer.
                bits.push(false);

                // Fill with tier bits.
                match self.uncommon_tier() {
                    // 00 for u8
                    UncommonShortValTier::U8 => {
                        bits.push(false);
                        bits.push(false);
                    }
                    // 01 for u16
                    UncommonShortValTier::U16 => {
                        bits.push(false);
                        bits.push(true);
                    }
                    // 10 for u24
                    UncommonShortValTier::U24 => {
                        bits.push(true);
                        bits.push(false);
                    }
                    // 11 for u32
                    UncommonShortValTier::U32 => {
                        bits.push(true);
                        bits.push(true);
                    }
                }

                // Fill with value bits.
                let value_bits = BitVec::from_bytes(&self.compact_bytes());
                bits.extend(value_bits);

                // Return the bits.
                bits
            }
        }
    }
}
