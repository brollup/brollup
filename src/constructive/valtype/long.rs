use crate::cpe::CompactPayloadEncoding;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LongVal(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LongValTier {
    U8,
    U16,
    U24,
    U32,
    U40,
    U48,
    U56,
    U64,
}

impl LongVal {
    /// Creates a new `LongVal`
    pub fn new(long_val: u64) -> Self {
        Self(long_val)
    }

    /// Returns the core u64 value.
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Determines the tier based on the value range.
    pub fn tier(&self) -> LongValTier {
        match self.value() {
            // 1 byte
            0..=0xFF => LongValTier::U8,
            // 2 bytes
            0x0100..=0xFFFF => LongValTier::U16,
            // 3 bytes
            0x00010000..=0xFFFFFF => LongValTier::U24,
            // 4 bytes
            0x0001000000..=0xFFFFFFFF => LongValTier::U32,
            // 5 bytes
            0x000100000000..=0xFFFFFFFFFF => LongValTier::U40,
            // 6 bytes
            0x00010000000000..=0xFFFFFFFFFFFF => LongValTier::U48,
            // 7 bytes
            0x0001000000000000..=0xFFFFFFFFFFFFFF => LongValTier::U56,
            // 8 bytes
            0x000100000000000000..=u64::MAX => LongValTier::U64,
        }
    }

    /// Returns the compact byte representation of the value.
    pub fn compact_bytes(&self) -> Vec<u8> {
        let value = self.value();
        let bytes = value.to_le_bytes();

        match self.tier() {
            // 1 byte
            LongValTier::U8 => vec![bytes[0]],
            // 2 bytes
            LongValTier::U16 => vec![bytes[0], bytes[1]],
            // 3 bytes
            LongValTier::U24 => vec![bytes[0], bytes[1], bytes[2]],
            // 4 bytes
            LongValTier::U32 => vec![bytes[0], bytes[1], bytes[2], bytes[3]],
            // 5 bytes
            LongValTier::U40 => vec![bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]],
            // 6 bytes
            LongValTier::U48 => vec![bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]],
            // 7 bytes
            LongValTier::U56 => vec![
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
            ],
            // 8 bytes
            LongValTier::U64 => bytes.to_vec(),
        }
    }

    /// Constructs a `LongVal` from its compact byte representation.
    pub fn from_compact_bytes(bytes: &[u8]) -> Option<Self> {
        let value = match bytes.len() {
            // 1 byte
            1 => bytes[0] as u64,
            // 2 bytes
            2 => u16::from_le_bytes([bytes[0], bytes[1]]) as u64,
            // 3 bytes
            3 => {
                let mut buf = [0u8; 8];
                buf[..3].copy_from_slice(bytes);
                u64::from_le_bytes(buf)
            }
            // 4 bytes
            4 => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64,
            // 5 bytes
            5 => {
                let mut buf = [0u8; 8];
                buf[..5].copy_from_slice(bytes);
                u64::from_le_bytes(buf)
            }
            // 6 bytes
            6 => {
                let mut buf = [0u8; 8];
                buf[..6].copy_from_slice(bytes);
                u64::from_le_bytes(buf)
            }
            // 7 bytes
            7 => {
                let mut buf = [0u8; 8];
                buf[..7].copy_from_slice(bytes);
                u64::from_le_bytes(buf)
            }
            // 8 bytes
            8 => u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
            _ => return None,
        };

        Some(Self(value))
    }
}

impl CompactPayloadEncoding for LongVal {
    fn encode(&self) -> BitVec {
        let mut bits = BitVec::new();

        // Fill with tier bits.
        match self.tier() {
            // 000 for u8
            LongValTier::U8 => {
                bits.push(false);
                bits.push(false);
                bits.push(false);
            }
            // 001 for u16
            LongValTier::U16 => {
                bits.push(false);
                bits.push(false);
                bits.push(true);
            }
            // 010 for u24
            LongValTier::U24 => {
                bits.push(false);
                bits.push(true);
                bits.push(false);
            }
            // 011 for u32
            LongValTier::U32 => {
                bits.push(false);
                bits.push(true);
                bits.push(true);
            }
            // 100 for u40
            LongValTier::U40 => {
                bits.push(true);
                bits.push(false);
                bits.push(false);
            }
            // 101 for u48
            LongValTier::U48 => {
                bits.push(true);
                bits.push(false);
                bits.push(true);
            }

            // 110 for u56
            LongValTier::U56 => {
                bits.push(true);
                bits.push(true);
                bits.push(false);
            }

            // 111 for u64
            LongValTier::U64 => {
                bits.push(true);
                bits.push(true);
                bits.push(true);
            }
        }

        // Fill with value bits.
        let value_bits = BitVec::from_bytes(&self.compact_bytes());
        bits.extend(value_bits);

        bits
    }

    fn decode(bits: &BitVec) -> Option<(LongVal, BitVec)> {
        let mut iter = bits.iter();

        let tier = match (iter.next()?, iter.next()?, iter.next()?) {
            // 000 for u8
            (false, false, false) => LongValTier::U8,
            // 001 for u16
            (false, false, true) => LongValTier::U16,
            // 010 for u24
            (false, true, false) => LongValTier::U24,
            // 011 for u32
            (false, true, true) => LongValTier::U32,
            // 100 for u40
            (true, false, false) => LongValTier::U40,
            // 101 for u48
            (true, false, true) => LongValTier::U48,
            // 110 for u56
            (true, true, false) => LongValTier::U56,
            // 111 for u64
            (true, true, true) => LongValTier::U64,
        };

        let bit_count = match tier {
            LongValTier::U8 => 8,
            LongValTier::U16 => 16,
            LongValTier::U24 => 24,
            LongValTier::U32 => 32,
            LongValTier::U40 => 40,
            LongValTier::U48 => 48,
            LongValTier::U56 => 56,
            LongValTier::U64 => 64,
        };

        let mut value_bits = BitVec::new();
        for _ in 0..bit_count {
            value_bits.push(iter.next()?);
        }

        let value_bytes = value_bits.to_bytes();
        let long_val = LongVal::from_compact_bytes(&value_bytes)?;

        let remaining_bits = iter.collect::<BitVec>();

        Some((long_val, remaining_bits))
    }
}
