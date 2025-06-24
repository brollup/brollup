use crate::constructive::valtype::maybe_common::common::common_long::cpe::decode::decode_error::CommonLongValCPEDecodingError;
use crate::constructive::valtype::maybe_common::common::common_short::cpe::decode::decode_error::CommonShortValCPEDecodingError;
use crate::constructive::valtype::val::long_val::cpe::decode::decode_error::LongValCPEDecodingError;
use crate::constructive::valtype::val::short_val::cpe::decode::decode_error::ShortValCPEDecodingError;

/// Enum to represent errors that can occur when decoding a `MaybeCommon` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MaybeCommonCPEDecodingError {
    IsCommonBitCollectError,
    CommonShortValCPEDecodingError(CommonShortValCPEDecodingError),
    CommonLongValCPEDecodingError(CommonLongValCPEDecodingError),
    ShortValCPEDecodingError(ShortValCPEDecodingError),
    LongValCPEDecodingError(LongValCPEDecodingError),
}
