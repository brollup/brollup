use crate::{
    cpe::{CPEError, CompactPayloadEncoding},
    registery::registery::REGISTERY,
};
use async_trait::async_trait;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShortVal(pub u32);

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

    /// Returns the core u32 value.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Determines the tier based on the value range.
    pub fn tier(&self) -> ShortValTier {
        match self.value() {
            0..=255 => ShortValTier::U8,
            256..=65535 => ShortValTier::U16,
            65536..=16777215 => ShortValTier::U24,
            16777216..=4294967295 => ShortValTier::U32,
        }
    }

    /// Returns the compact byte representation of the value.
    pub fn compact_bytes(&self) -> Vec<u8> {
        let value = self.value();

        match self.tier() {
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
}

#[async_trait]
impl CompactPayloadEncoding for ShortVal {
    fn encode(&self) -> BitVec {
        let mut bits = BitVec::new();

        // Fill with tier bits.
        match self.tier() {
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

        bits
    }

    async fn decode(
        bits: &BitVec,
        _registery: Option<&REGISTERY>,
    ) -> Result<(ShortVal, BitVec), CPEError> {
        let mut iter = bits.iter();

        let tier = match (iter.next(), iter.next()) {
            (Some(false), Some(false)) => ShortValTier::U8,
            (Some(false), Some(true)) => ShortValTier::U16,
            (Some(true), Some(false)) => ShortValTier::U24,
            (Some(true), Some(true)) => ShortValTier::U32,
            _ => return Err(CPEError::IteratorError),
        };

        let bit_count = match tier {
            ShortValTier::U8 => 8,
            ShortValTier::U16 => 16,
            ShortValTier::U24 => 24,
            ShortValTier::U32 => 32,
        };

        let mut value_bits = BitVec::new();
        for _ in 0..bit_count {
            value_bits.push(iter.next().ok_or(CPEError::IteratorError)?);
        }

        let value_bytes = value_bits.to_bytes();
        let short_val =
            ShortVal::from_compact_bytes(&value_bytes).ok_or(CPEError::ConversionError)?;

        let remaining_bits = iter.collect::<BitVec>();

        Ok((short_val, remaining_bits))
    }
}
