use crate::constructive::valtype::val::short_val::short_val::{ShortVal, ShortValTier};
use bit_vec::BitVec;

impl ShortVal {
    /// Encodes a `ShortVal` into a bit vector.
    pub fn encode_cpe(&self) -> BitVec {
        // Initialize the bit vector.
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

        // Convert the compact bytes to bits.
        let value_bits = BitVec::from_bytes(&self.compact_bytes());

        // Extend the bits with the value bits.
        bits.extend(value_bits);

        // Return the bits.
        bits
    }
}
