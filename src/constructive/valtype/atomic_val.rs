use crate::constructive::cpe::{
    cpe::CompactPayloadEncoding,
    decode_error::{error::CPEDecodingError, valtype_error::AtomicValCPEDecodingError},
};
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// The inner value `AtomicVal` represents.
type Value = u8;

/// The upper bound of the `AtomicVal`.
type UpperBound = u8;

/// Atomic compact value representation from 0 to 255.
///
/// Used for very small value representations such as contract method call indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AtomicVal(Value, UpperBound);

impl AtomicVal {
    /// Creates a new `AtomicVal`
    pub fn new(value: Value, upper_bound: UpperBound) -> Self {
        Self(value, upper_bound)
    }

    /// Returns the core u8 value.
    pub fn value(&self) -> Value {
        self.0
    }

    /// Returns the upper bound.
    pub fn upper_bound(&self) -> UpperBound {
        self.1
    }

    /// Returns the bitsize tier of `AtomicVal`.
    fn bitsize(upper_bound: UpperBound) -> u8 {
        match upper_bound {
            0..=1 => 1,
            2..=3 => 2,
            4..=7 => 3,
            8..=15 => 4,
            16..=31 => 5,
            32..=63 => 6,
            64..=127 => 7,
            128..=255 => 8,
        }
    }

    /// Convert a u8 to a n-bit BitVec.
    fn u8_to_bits(value: u8, bitsize: u8) -> BitVec {
        let val_bytes = value.to_le_bytes();

        // Initialize a BitVec.
        let mut val_bits = BitVec::new();

        // Iterate bitsize number of times.
        for i in 0..bitsize {
            val_bits.push((val_bytes[0] >> i) & 1 == 1);
        }

        // Return the BitVec.
        val_bits
    }

    /// Converts bits to a u8.
    fn bits_to_u8(bits: &BitVec, bitsize: u8) -> u8 {
        // Initialize a u8.
        let mut decoded_val = 0u8;

        // Iterate bitsize number of times.
        for i in 0..bitsize {
            let bit = bits[i as usize];
            if bit {
                decoded_val |= 1 << i;
            }
        }

        // Return the u8.
        decoded_val
    }

    /// Compact payload decoding for `AtomicVal`.
    /// Decodes an `AtomicVal` from a bit stream.
    pub fn decode_cpe(
        bit_stream: &mut bit_vec::Iter<'_>,
        upper_bound: u8,
    ) -> Result<AtomicVal, CPEDecodingError> {
        // Initialize a BitVec.
        let mut bits = BitVec::new();

        // Determine the bitsize of the `AtomicVal`.
        let bitsize = AtomicVal::bitsize(upper_bound);

        // Collect bitsize number of bits.
        for _ in 0..bitsize {
            bits.push(
                bit_stream
                    .next()
                    .ok_or(CPEDecodingError::AtomicValCPEDecodingError(
                        AtomicValCPEDecodingError::BitStreamIteratorError,
                    ))?,
            );
        }

        // Convert the collected bits to a u8 value.
        let value = AtomicVal::bits_to_u8(&bits, bitsize);

        // Convert the u8 value to an `AtomicVal`.
        let atomic_val = AtomicVal::new(value, upper_bound);

        // Return the `AtomicVal`.
        Ok(atomic_val)
    }
}

impl CompactPayloadEncoding for AtomicVal {
    /// Compact payload encoding for `AtomicVal`.
    /// Encodes an `AtomicVal` to a bit stream.
    fn encode_cpe(&self) -> Option<BitVec> {
        // Determine the bitsize of the `AtomicVal`.
        let bitsize = AtomicVal::bitsize(self.upper_bound());

        // Convert the value to a n-bit BitVec.
        let bits = AtomicVal::u8_to_bits(self.value(), bitsize);

        // Return the BitVec.
        Some(bits)
    }
}
