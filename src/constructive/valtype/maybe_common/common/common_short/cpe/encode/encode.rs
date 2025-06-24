use crate::constructive::valtype::maybe_common::common::common_short::common_short::{
    CommonShortVal, COMMON_SHORT_BITSIZE,
};
use crate::constructive::valtype::maybe_common::common::common_short::cpe::encode::encode_error::CommonShortValCPEEncodeError;
use crate::constructive::valtype::u8_ext::U8Ext;
use bit_vec::BitVec;

impl CommonShortVal {
    /// Encodes a `CommonShortVal` into a bit vector.
    pub fn encode_cpe(&self) -> Result<BitVec, CommonShortValCPEEncodeError> {
        // Get the index.
        let index = self.index();

        // Convert index to a bits with the bitsize of 6.
        let bits = u8::to_bits(index, COMMON_SHORT_BITSIZE)
            .ok_or(CommonShortValCPEEncodeError::U8ExtToBitsError)?;

        // Return the bits.
        Ok(bits)
    }
}
