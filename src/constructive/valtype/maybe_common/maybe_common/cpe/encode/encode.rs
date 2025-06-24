use crate::constructive::valtype::{
    maybe_common::maybe_common::{
        cpe::encode::encode_error::MaybeCommonCPEEncodeError,
        maybe_common::{CommonVal, Commonable, MaybeCommon, MaybeCommonValue},
    },
    val::{long_val::long_val::LongVal, short_val::short_val::ShortVal},
};
use bit_vec::BitVec;

impl<T> MaybeCommon<T>
where
    T: Commonable + Clone + From<ShortVal> + From<LongVal>,
{
    /// Encodes a `MaybeCommon` into a bit vector.
    pub fn encode_cpe(&self) -> Result<BitVec, MaybeCommonCPEEncodeError> {
        // Create a new BitVec.
        let mut bits = BitVec::new();

        match self {
            MaybeCommon::Common(common_val) => {
                // Insert true for common.
                bits.push(true);

                // Insert the common value.
                match common_val {
                    CommonVal::CommonShort(common_short_val) => {
                        // Extend 6 bits.
                        bits.extend(common_short_val.encode_cpe().map_err(|e| {
                            MaybeCommonCPEEncodeError::CommonShortValCPEEncodeError(e)
                        })?);
                    }
                    CommonVal::CommonLong(common_long_val) => {
                        // Extend 7 bits.
                        bits.extend(common_long_val.encode_cpe().map_err(|e| {
                            MaybeCommonCPEEncodeError::CommonLongValCPEEncodeError(e)
                        })?);
                    }
                }

                // Return the bits.
                Ok(bits)
            }
            MaybeCommon::Uncommon(uncommon_val) => {
                // Insert false for uncommon.
                bits.push(false);

                // Encode the uncommon value based on its type
                match uncommon_val.maybe_common_value() {
                    MaybeCommonValue::Short(short_val) => {
                        // Encode as ShortVal
                        bits.extend(short_val.encode_cpe());
                    }
                    MaybeCommonValue::Long(long_val) => {
                        // Encode as LongVal
                        bits.extend(long_val.encode_cpe());
                    }
                }

                // Return the bits.
                Ok(bits)
            }
        }
    }
}
