use crate::constructive::valtype::maybe_common::common::common_long::common_long::CommonLongVal;
use crate::constructive::valtype::maybe_common::common::common_short::common_short::CommonShortVal;
use crate::constructive::valtype::maybe_common::maybe_common::cpe::decode::decode_error::MaybeCommonCPEDecodingError;
use crate::constructive::valtype::maybe_common::maybe_common::maybe_common::MaybeCommonValueType;
use crate::constructive::valtype::maybe_common::maybe_common::maybe_common::{
    CommonVal, Commonable, MaybeCommon,
};
use crate::constructive::valtype::val::long_val::long_val::LongVal;
use crate::constructive::valtype::val::short_val::short_val::ShortVal;

impl<T> MaybeCommon<T>
where
    T: Commonable + Clone + From<ShortVal> + From<LongVal>,
{
    /// Decodes a `MaybeCommon` from a bit stream.
    pub fn decode_cpe(
        bit_stream: &mut bit_vec::Iter<'_>,
    ) -> Result<MaybeCommon<T>, MaybeCommonCPEDecodingError> {
        // Check if the value is common.
        let is_common = bit_stream
            .next()
            .ok_or(MaybeCommonCPEDecodingError::IsCommonBitCollectError)?;

        match is_common {
            true => {
                // Value is common.

                // Check if the common value is short or long
                match T::maybe_common_value_type() {
                    MaybeCommonValueType::Short => {
                        // Decode common short value from 6 bits.
                        let common_short_val =

                        
                            CommonShortVal::decode_cpe(bit_stream).map_err(|e| {
                                MaybeCommonCPEDecodingError::CommonShortValCPEDecodingError(e)
                            })?;

                        // Return the common short value.
                        Ok(MaybeCommon::Common(CommonVal::CommonShort(
                            common_short_val,
                        )))
                    }
                    MaybeCommonValueType::Long => {
                        // Decode common long value from 7 bits.
                        let common_long_val =
                            CommonLongVal::decode_cpe(bit_stream).map_err(|e| {
                                MaybeCommonCPEDecodingError::CommonLongValCPEDecodingError(e)
                            })?;

                        // Return the common long value.
                        Ok(MaybeCommon::Common(CommonVal::CommonLong(common_long_val)))
                    }
                }
            }
            false => {
                // Value is uncommon.

                // Check if the uncommon value is short or long
                match T::maybe_common_value_type() {
                    MaybeCommonValueType::Short => {
                        // Decode uncommon short value.
                        let uncommon_short_val = ShortVal::decode_cpe(bit_stream).map_err(|e| {
                            MaybeCommonCPEDecodingError::ShortValCPEDecodingError(e)
                        })?;

                        // Return the uncommon short value.
                        Ok(MaybeCommon::Uncommon(uncommon_short_val.into()))
                    }
                    MaybeCommonValueType::Long => {
                        // Decode uncommon long value.
                        let uncommon_long_val = LongVal::decode_cpe(bit_stream)
                            .map_err(|e| MaybeCommonCPEDecodingError::LongValCPEDecodingError(e))?;

                        // Return the uncommon long value.
                        Ok(MaybeCommon::Uncommon(uncommon_long_val.into()))
                    }
                }
            }
        }
    }
}
