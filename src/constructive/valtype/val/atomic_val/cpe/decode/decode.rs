use crate::constructive::valtype::u8_ext::U8Ext;
use crate::constructive::valtype::val::atomic_val::atomic_val::AtomicVal;
use crate::constructive::valtype::val::atomic_val::cpe::decode::decode_error::AtomicValCPEDecodingError;
use bit_vec::BitVec;

impl AtomicVal {
    /// Compact payload decoding for `AtomicVal`.
    /// Decodes an `AtomicVal` from a bit stream.
    pub fn decode_cpe(
        bit_stream: &mut bit_vec::Iter<'_>,
        upper_bound: u8,
    ) -> Result<AtomicVal, AtomicValCPEDecodingError> {
        // Initialize a BitVec.
        let mut bits = BitVec::new();

        // Determine the bitsize of the `AtomicVal`.
        let bitsize = AtomicVal::bitsize(upper_bound);

        // Collect bitsize number of bits.
        for _ in 0..bitsize {
            bits.push(
                bit_stream
                    .next()
                    .ok_or(AtomicValCPEDecodingError::CollectBitsizeNumberBitsError)?,
            );
        }

        // Convert the collected bits to a u8 value.
        let value =
            u8::from_bits(&bits, bitsize).ok_or(AtomicValCPEDecodingError::U8ExtFromBitsError)?;

        // Convert the u8 value to an `AtomicVal`.
        let atomic_val = AtomicVal::new(value, upper_bound);

        // Return the `AtomicVal`.
        Ok(atomic_val)
    }
}
