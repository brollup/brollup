use crate::constructive::valtype::val::long_val::long_val::{LongVal, LongValTier};
use bit_vec::BitVec;

impl LongVal {
    /// Encodes a `LongVal` into a bit vector.
    pub fn encode_cpe(&self) -> BitVec {
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

        // Convert the compact bytes to bits.
        let value_bits = BitVec::from_bytes(&self.compact_bytes());

        // Extend the bits with the value bits.
        bits.extend(value_bits);

        // Return the bits.
        bits
    }
}
