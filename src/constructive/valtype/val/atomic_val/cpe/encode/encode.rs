use crate::constructive::valtype::u8_ext::U8Ext;
use crate::constructive::valtype::val::atomic_val::atomic_val::AtomicVal;
use crate::constructive::valtype::val::atomic_val::cpe::encode::encode_error::AtomicValCPEEncodeError;
use bit_vec::BitVec;

impl AtomicVal {
    /// Compact payload encoding for `AtomicVal`.
    /// Encodes an `AtomicVal` to a bit stream.
    pub fn encode_cpe(&self) -> Result<BitVec, AtomicValCPEEncodeError> {
        // Determine the bitsize of the `AtomicVal`.
        let bitsize = AtomicVal::bitsize(self.upper_bound());

        // Convert the value to a n-bit BitVec.
        let bits =
            u8::to_bits(self.value(), bitsize).ok_or(AtomicValCPEEncodeError::U8ExtToBitsError)?;

        // Return the BitVec.
        Ok(bits)
    }
}
