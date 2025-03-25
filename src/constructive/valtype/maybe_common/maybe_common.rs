use super::common::CommonVal;
use crate::{
    cpe::{CPEDecodingError, CompactPayloadEncoding, MaybeCommonCPEDecodingError},
    valtype::{long::LongVal, short::ShortVal},
};
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// Trait to determine whether a value is `ShortOrLong`.
pub trait Commonable {
    fn maybe_common_type(&self) -> MaybeCommonType;
}

/// Enum to represent either `ShortVal` or `LongVal`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaybeCommonType {
    Short(ShortVal),
    Long(LongVal),
}

/// Enum to represent either `ShortVal` or `LongVal`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortOrLong {
    Short,
    Long,
}

/// Represents a generic `MaybeCommon` struct that encapsulates either a "common" or an "uncommon" value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaybeCommon<T> {
    Common(CommonVal),
    Uncommon(T),
}

impl<T> MaybeCommon<T>
where
    T: Commonable + CompactPayloadEncoding + Clone,
{
    /// Creates a new `MaybeCommon` for ShortVal or LongVal.
    pub fn new(val: T) -> Self {
        match val.maybe_common_type() {
            MaybeCommonType::Short(short_val) => {
                // Check if the value is common.
                match CommonVal::new(short_val.value()) {
                    Some(common_val) => MaybeCommon::Common(common_val),
                    None => MaybeCommon::Uncommon(val),
                }
            }
            MaybeCommonType::Long(long_val) => {
                // Check if the value is within u32 range.
                match long_val.value() <= u32::MAX as u64 {
                    true => {
                        // Check if the value is common.
                        match CommonVal::new(long_val.value() as u32) {
                            Some(common_val) => MaybeCommon::Common(common_val),
                            None => MaybeCommon::Uncommon(val),
                        }
                    }
                    // If the value is not within u32 range, it is uncommon.
                    false => MaybeCommon::Uncommon(val),
                }
            }
        }
    }

    /// Returns the u64 value of the `MaybeCommon` struct.
    pub fn value(&self) -> u64 {
        match self {
            MaybeCommon::Common(common_val) => common_val.value() as u64,
            MaybeCommon::Uncommon(uncommon_val) => match uncommon_val.maybe_common_type() {
                MaybeCommonType::Short(short_val) => short_val.value() as u64,
                MaybeCommonType::Long(long_val) => long_val.value(),
            },
        }
    }

    /// Decodes a `MaybeCommon` from a bit stream.
    pub fn decode_cpe(
        bit_stream: &mut bit_vec::Iter<'_>,
        short_or_long: ShortOrLong,
    ) -> Result<Self, CPEDecodingError>
    where
        T: From<ShortVal> + From<LongVal>, // Ensure T can be created from ShortVal or LongVal
    {
        // Check if the value is common.
        let is_common = bit_stream
            .next()
            .ok_or(CPEDecodingError::MaybeCommonCPEDecodingError(
                MaybeCommonCPEDecodingError::BitStreamIteratorError,
            ))?;

        // Match is common or uncommon.
        match is_common {
            // If the value is common, decode the common value.
            true => {
                let common_val = CommonVal::decode_cpe(bit_stream).map_err(|_| {
                    CPEDecodingError::MaybeCommonCPEDecodingError(
                        MaybeCommonCPEDecodingError::CommonValCPEDecodingError,
                    )
                })?;
                Ok(MaybeCommon::Common(common_val))
            }
            // If the value is uncommon, decode the uncommon value.
            false => match short_or_long {
                ShortOrLong::Short => {
                    let uncommon_val = ShortVal::decode_cpe(bit_stream).map_err(|_| {
                        CPEDecodingError::MaybeCommonCPEDecodingError(
                            MaybeCommonCPEDecodingError::ShortUncommonValCPEDecodingError,
                        )
                    })?;
                    Ok(MaybeCommon::Uncommon(uncommon_val.into()))
                }
                ShortOrLong::Long => {
                    let uncommon_val = LongVal::decode_cpe(bit_stream).map_err(|_| {
                        CPEDecodingError::MaybeCommonCPEDecodingError(
                            MaybeCommonCPEDecodingError::LongUncommonValCPEDecodingError,
                        )
                    })?;
                    Ok(MaybeCommon::Uncommon(uncommon_val.into()))
                }
            },
        }
    }
}

/// Compact payload encoding for `MaybeCommon`.
impl<T> CompactPayloadEncoding for MaybeCommon<T>
where
    T: Commonable + CompactPayloadEncoding + Clone,
{
    fn encode_cpe(&self) -> BitVec {
        // Create a new `BitVec`.
        let mut bits = BitVec::new();

        match self {
            MaybeCommon::Common(common_val) => {
                // Insert true for common.
                bits.push(true);

                // Insert the common value.
                bits.extend(common_val.encode_cpe());

                // Return the bits.
                bits
            }
            MaybeCommon::Uncommon(uncommon_val) => {
                // Insert false for uncommon.
                bits.push(false);

                // Insert the uncommon value.
                bits.extend(uncommon_val.encode_cpe());

                // Return the bits.
                bits
            }
        }
    }
}
