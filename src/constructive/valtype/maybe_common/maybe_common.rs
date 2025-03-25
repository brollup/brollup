use super::common_val::CommonVal;
use crate::{
    cpe::{CPEDecodingError, CompactPayloadEncoding, MaybeCommonCPEDecodingError},
    valtype::{long_val::LongVal, short_val::ShortVal},
};
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// Trait to determine whether a value is ShortOrLong.
pub trait Commonable {
    fn maybe_common_type(&self) -> MaybeCommonType;
    /// Associated function to return the type name as a string.
    fn short_or_long() -> ShortOrLong
    where
        Self: Sized;
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
    T: Commonable + CompactPayloadEncoding + Clone + From<ShortVal> + From<LongVal>,
{
    /// Creates a new `MaybeCommon` for `ShortVal` or `LongVal`.
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

    /// Returns the inner value which is either a `ShortVal` or a `LongVal`.
    pub fn inner_val(&self) -> T {
        match self {
            MaybeCommon::Common(val) => match T::short_or_long() {
                ShortOrLong::Short => ShortVal::new(val.value()).into(),
                ShortOrLong::Long => LongVal::new(val.value() as u64).into(),
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

        // If the value is common, decode it as `CommonVal`.
        if is_common {
            let common_val = CommonVal::decode_cpe(bit_stream).map_err(|_| {
                CPEDecodingError::MaybeCommonCPEDecodingError(
                    MaybeCommonCPEDecodingError::CommonValCPEDecodingError,
                )
            })?;
            return Ok(MaybeCommon::Common(common_val));
        }

        // If the value is uncommon, decode it based on the type `T`.
        match T::short_or_long() {
            ShortOrLong::Short => {
                let uncommon_val = ShortVal::decode_cpe(bit_stream).map_err(|_| {
                    CPEDecodingError::MaybeCommonCPEDecodingError(
                        MaybeCommonCPEDecodingError::ShortUncommonValCPEDecodingError,
                    )
                })?;
                Ok(MaybeCommon::Uncommon(T::from(uncommon_val)))
            }
            ShortOrLong::Long => {
                let uncommon_val = LongVal::decode_cpe(bit_stream).map_err(|_| {
                    CPEDecodingError::MaybeCommonCPEDecodingError(
                        MaybeCommonCPEDecodingError::LongUncommonValCPEDecodingError,
                    )
                })?;
                Ok(MaybeCommon::Uncommon(T::from(uncommon_val)))
            }
        }
    }
}

/// Compact payload encoding for `MaybeCommon`.
impl<T> CompactPayloadEncoding for MaybeCommon<T>
where
    T: Commonable + CompactPayloadEncoding + Clone + From<ShortVal> + From<LongVal>,
{
    fn encode_cpe(&self) -> BitVec {
        // Create a new BitVec.
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
