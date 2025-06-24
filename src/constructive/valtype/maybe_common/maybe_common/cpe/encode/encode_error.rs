use crate::constructive::valtype::maybe_common::common::{
    common_long::cpe::encode::encode_error::CommonLongValCPEEncodeError,
    common_short::cpe::encode::encode_error::CommonShortValCPEEncodeError,
};

/// Enum to represent errors that can occur when encoding a `MaybeCommon` into a bit vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MaybeCommonCPEEncodeError {
    CommonShortValCPEEncodeError(CommonShortValCPEEncodeError),
    CommonLongValCPEEncodeError(CommonLongValCPEEncodeError),
}
