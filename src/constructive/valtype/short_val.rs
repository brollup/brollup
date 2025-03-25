use super::long_val::LongVal;
use super::maybe_common::maybe_common::{Commonable, MaybeCommonType, ShortOrLong};
use crate::cpe::{CPEDecodingError, CompactPayloadEncoding, ShortValCPEDecodingError};
use async_trait::async_trait;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShortVal(pub u32);

/// Represents the tier of an uncommon `ShortVal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShortValTier {
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

    /// Returns the core value as a u64.
    pub fn value(&self) -> u64 {
        self.0 as u64
    }

    pub fn value_u32(&self) -> u32 {
        self.0
    }

    /// Determines the tier based on the value range.
    pub fn uncommon_tier(&self) -> ShortValTier {
        match self.0 {
            0..=255 => ShortValTier::U8,
            256..=65535 => ShortValTier::U16,
            65536..=16777215 => ShortValTier::U24,
            16777216..=4294967295 => ShortValTier::U32,
        }
    }

    /// Returns the compact byte representation of the value.
    pub fn compact_bytes(&self) -> Vec<u8> {
        let value = self.value_u32();

        match self.uncommon_tier() {
            // 1 byte
            ShortValTier::U8 => vec![value as u8],
            // 2 bytes
            ShortValTier::U16 => {
                let bytes = value.to_le_bytes();
                vec![bytes[0], bytes[1]]
            }
            // 3 bytes
            ShortValTier::U24 => {
                let bytes = value.to_le_bytes();
                vec![bytes[0], bytes[1], bytes[2]]
            }
            // 4 bytes
            ShortValTier::U32 => value.to_le_bytes().to_vec(),
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
        // Decode the  tier.
        let tier = match (bit_stream.next(), bit_stream.next()) {
            (Some(false), Some(false)) => ShortValTier::U8,
            (Some(false), Some(true)) => ShortValTier::U16,
            (Some(true), Some(false)) => ShortValTier::U24,
            (Some(true), Some(true)) => ShortValTier::U32,
            _ => {
                return Err(CPEDecodingError::ShortValCPEDecodingError(
                    ShortValCPEDecodingError::BitStreamIteratorError,
                ))
            }
        };

        // Get the bit count for the tier.
        let bit_count = match tier {
            ShortValTier::U8 => 8,
            ShortValTier::U16 => 16,
            ShortValTier::U24 => 24,
            ShortValTier::U32 => 32,
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

#[async_trait]
impl CompactPayloadEncoding for ShortVal {
    fn encode_cpe(&self) -> BitVec {
        let mut bits = BitVec::new();
        // Fill with tier bits.
        match self.uncommon_tier() {
            // 00 for u8
            ShortValTier::U8 => {
                bits.push(false);
                bits.push(false);
            }
            // 01 for u16
            ShortValTier::U16 => {
                bits.push(false);
                bits.push(true);
            }
            // 10 for u24
            ShortValTier::U24 => {
                bits.push(true);
                bits.push(false);
            }
            // 11 for u32
            ShortValTier::U32 => {
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

/// Implement `Commonable` for `ShortVal`.
impl Commonable for ShortVal {
    fn maybe_common_type(&self) -> MaybeCommonType {
        MaybeCommonType::Short(self.clone())
    }

    fn short_or_long() -> ShortOrLong {
        ShortOrLong::Short
    }
}

/// Implement `From` for `ShortVal` from `LongVal`.
impl From<LongVal> for ShortVal {
    fn from(val: LongVal) -> Self {
        ShortVal::new(val.value() as u32)
    }
}
