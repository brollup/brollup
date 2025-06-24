use crate::constructive::valtype::val::short_val::cpe::decode::decode_error::ShortValCPEDecodingError;

/// Type alias for the account rank.
type AccountRank = u32;

/// Enum to represent errors that can occur when decoding an `Account` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountCPEDecodingError {
    RankAsShortValDecodeError(ShortValCPEDecodingError),
    PublicKeyBitsLengthError,
    PublicKeyPointFromSliceError,
    KeyAlreadyRegisteredError,
    FailedToLocateAccountGivenRank(AccountRank),
}
