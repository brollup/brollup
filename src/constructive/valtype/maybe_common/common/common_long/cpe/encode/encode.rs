use crate::constructive::valtype::maybe_common::common::common_long::{
    common_long::{CommonLongVal, COMMON_LONG_BITSIZE},
    cpe::encode::encode_error::CommonLongValCPEEncodeError,
};
use crate::constructive::valtype::u8_ext::U8Ext;
use bit_vec::BitVec;

impl CommonLongVal {
    /// Encodes a `CommonLongVal` into a bit vector.
    pub fn encode_cpe(&self) -> Result<BitVec, CommonLongValCPEEncodeError> {
        // Get the index.
        let index = self.index();

        // Convert index to a bits with the bitsize of 7.
        let bits = u8::to_bits(index, COMMON_LONG_BITSIZE)
            .ok_or(CommonLongValCPEEncodeError::U8ExtToBitsError)?;

        // Return the bits.
        Ok(bits)
    }
}
