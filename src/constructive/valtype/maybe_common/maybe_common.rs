use super::{common_long::CommonLongVal, common_short::CommonShortVal};
use crate::constructive::cpe::{
    cpe::CompactPayloadEncoding,
    decode_error::{error::CPEDecodingError, valtype_error::MaybeCommonCPEDecodingError},
};
use crate::constructive::valtype::long_val::LongVal;
use crate::constructive::valtype::short_val::ShortVal;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// Trait to determine whether a value is ShortOrLong.
pub trait Commonable {
    fn maybe_common_value(&self) -> MaybeCommonValue;
    /// Associated function to return the type name as a string.
    fn maybe_common_value_type() -> MaybeCommonValueType
    where
        Self: Sized;
}

/// Enum to represent either `ShortVal` or `LongVal`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaybeCommonValueType {
    Short,
    Long,
}

/// Enum to represent either `ShortVal` or `LongVal`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaybeCommonValue {
    Short(ShortVal),
    Long(LongVal),
}

/// Represents a common value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommonVal {
    CommonShort(CommonShortVal),
    CommonLong(CommonLongVal),
}

/// Represents a generic `MaybeCommon` struct that encapsulates either a "common" or an "uncommon" value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaybeCommon<T> {
    Common(CommonVal),
    Uncommon(T),
}

impl<T> MaybeCommon<T>
where
    T: Commonable + CompactPayloadEncoding + Clone + From<ShortVal> + From<LongVal>,
{
    /// Creates a new `MaybeCommon` for `ShortVal` or `LongVal`.
    pub fn new(val: T) -> Self {
        match val.maybe_common_value() {
            MaybeCommonValue::Short(short_val) => {
                // Check if the value is common.
                match CommonShortVal::new(short_val.value()) {
                    Some(common_val) => MaybeCommon::Common(CommonVal::CommonShort(common_val)),
                    None => MaybeCommon::Uncommon(val),
                }
            }
            MaybeCommonValue::Long(long_val) => {
                // Check if the value is within u32 range.
                match long_val.value() <= u32::MAX as u64 {
                    true => {
                        // Check if the value is common.
                        match CommonLongVal::new(long_val.value()) {
                            Some(common_val) => {
                                MaybeCommon::Common(CommonVal::CommonLong(common_val))
                            }
                            None => MaybeCommon::Uncommon(val),
                        }
                    }
                    // If the value is not within u32 range, it is uncommon.
                    false => MaybeCommon::Uncommon(val),
                }
            }
        }
    }

    /// Returns the inner value as either a `ShortVal` or a `LongVal`.
    pub fn value(&self) -> T {
        match self {
            MaybeCommon::Common(val) => match val {
                CommonVal::CommonShort(common_short_val) => {
                    ShortVal::new(common_short_val.value()).into()
                }
                CommonVal::CommonLong(common_long_val) => {
                    LongVal::new(common_long_val.value()).into()
                }
            },
            MaybeCommon::Uncommon(val) => val.to_owned(),
        }
    }

    /// Returns true if the `MaybeCommon` struct is common.
    pub fn is_common(&self) -> bool {
        match self {
            MaybeCommon::Common(_) => true,
            MaybeCommon::Uncommon(_) => false,
        }
    }

    /// Decodes a `MaybeCommon` from a bit stream.
    pub fn decode_cpe(bit_stream: &mut bit_vec::Iter<'_>) -> Result<Self, CPEDecodingError> {
        // Check if the value is common.
        let is_common = bit_stream
            .next()
            .ok_or(CPEDecodingError::MaybeCommonCPEDecodingError(
                MaybeCommonCPEDecodingError::BitStreamIteratorError,
            ))?;

        match is_common {
            true => {
                // Value is common.

                // Check if the common value is short or long
                match T::maybe_common_value_type() {
                    MaybeCommonValueType::Short => {
                        // Decode common short value from 6 bits.
                        let common_short_val = CommonShortVal::decode_cpe(bit_stream)?;

                        // Return the common short value.
                        Ok(MaybeCommon::Common(CommonVal::CommonShort(
                            common_short_val,
                        )))
                    }
                    MaybeCommonValueType::Long => {
                        // Decode common long value from 7 bits.
                        let common_long_val = CommonLongVal::decode_cpe(bit_stream)?;

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
                        let uncommon_short_val =
                            ShortVal::decode_cpe(bit_stream).map_err(|e| {
                                match e {
                                    CPEDecodingError::ShortValCPEDecodingError(err) => {
                                        CPEDecodingError::MaybeCommonCPEDecodingError(
                                            MaybeCommonCPEDecodingError::UncommonShortValCPEDecodingError(err),
                                        )
                                    },
                                    _ => CPEDecodingError::UnexpectedError,
                                }
                                
                            })?;

                        // Return the uncommon short value.
                        Ok(MaybeCommon::Uncommon(uncommon_short_val.into()))
                    }
                    MaybeCommonValueType::Long => {
                        // Decode uncommon long value.
                        let uncommon_long_val = LongVal::decode_cpe(bit_stream).map_err(|e| {
                            match e {
                                CPEDecodingError::LongValCPEDecodingError(err) => {
                                    CPEDecodingError::MaybeCommonCPEDecodingError(
                                        MaybeCommonCPEDecodingError::UncommonLongValCPEDecodingError(err),
                                    )
                                },
                                _ => CPEDecodingError::UnexpectedError,
                            }
                            
                        })?;

                        // Return the uncommon long value.
                        Ok(MaybeCommon::Uncommon(uncommon_long_val.into()))
                    }
                }
            }
        }
    }
}

/// Compact payload encoding for `MaybeCommon`.
impl<T> CompactPayloadEncoding for MaybeCommon<T>
where
    T: Commonable + CompactPayloadEncoding + Clone + From<ShortVal> + From<LongVal>,
{
    fn encode_cpe(&self) -> Option<BitVec> {
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
                        bits.extend(common_short_val.encode_cpe()?);
                    }
                    CommonVal::CommonLong(common_long_val) => {
                        // Extend 7 bits.
                        bits.extend(common_long_val.encode_cpe()?);
                    }
                }

                // Return the bits.
                Some(bits)
            }
            MaybeCommon::Uncommon(uncommon_val) => {
                // Insert false for uncommon.
                bits.push(false);

                // Insert the uncommon value.
                bits.extend(uncommon_val.encode_cpe()?);

                // Return the bits.
                Some(bits)
            }
        }
    }
}
