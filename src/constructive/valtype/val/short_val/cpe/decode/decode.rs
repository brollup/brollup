use crate::constructive::valtype::val::short_val::{
    cpe::decode::decode_error::ShortValCPEDecodingError,
    short_val::{ShortVal, ShortValTier},
};
use bit_vec::BitVec;

/// Compact payload decoding for `ShortVal`.
impl ShortVal {
    /// Decodes a `ShortVal` from a bit stream.
    pub fn decode_cpe(
        bit_stream: &mut bit_vec::Iter<'_>,
    ) -> Result<ShortVal, ShortValCPEDecodingError> {
        // Decode the  tier.
        let tier = match (bit_stream.next(), bit_stream.next()) {
            (Some(false), Some(false)) => ShortValTier::U8,
            (Some(false), Some(true)) => ShortValTier::U16,
            (Some(true), Some(false)) => ShortValTier::U24,
            (Some(true), Some(true)) => ShortValTier::U32,
            _ => {
                return Err(ShortValCPEDecodingError::TierBitsCollectError);
            }
        };

        // Get the bit count for the tier.
        let bit_count = match tier {
            ShortValTier::U8 => 8,
            ShortValTier::U16 => 16,
            ShortValTier::U24 => 24,
            ShortValTier::U32 => 32,
        };

        // Initialize the value bits.
        let mut value_bits = BitVec::new();

        // Collect the value bits.
        for _ in 0..bit_count {
            value_bits.push(
                bit_stream
                    .next()
                    .ok_or(ShortValCPEDecodingError::ValueBitsCollectError)?,
            );
        }

        // Convert the value bits to bytes.
        let value_bytes = value_bits.to_bytes();

        // Construct the short value.
        let short_val = ShortVal::from_compact_bytes(&value_bytes)
            .ok_or(ShortValCPEDecodingError::ShortValFromCompactBytesConstructionError)?;

        // Return the `ShortVal`.
        Ok(short_val)
    }
}
