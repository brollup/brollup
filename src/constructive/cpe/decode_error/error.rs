use super::{
    calldata_error::CalldataCPEDecodingError,
    entity_error::{AccountCPEDecodingError, ContractCPEDecodingError},
    entry_error::LiftupCPEDecodingError,
    valtype_error::{
        AtomicValCPEDecodingError, LongValCPEDecodingError, MaybeCommonCPEDecodingError,
        ShortValCPEDecodingError,
    },
};
use serde::{Deserialize, Serialize};

/// Error type for compact payload decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CPEDecodingError {
    // Maybe common CPE decoding error.
    MaybeCommonCPEDecodingError(MaybeCommonCPEDecodingError),
    // Atomic value CPE decoding error.
    AtomicValCPEDecodingError(AtomicValCPEDecodingError),
    // Short value CPE decoding error.
    ShortValCPEDecodingError(ShortValCPEDecodingError),
    // Long value CPE decoding error.
    LongValCPEDecodingError(LongValCPEDecodingError),
    // Account CPE decoding error.
    AccountCPEDecodingError(AccountCPEDecodingError),
    // Contract CPE decoding error.
    ContractCPEDecodingError(ContractCPEDecodingError),
    // Liftup CPE decoding error.
    LiftupCPEDecodingError(LiftupCPEDecodingError),
    // Calldata CPE decoding error.
    CalldataCPEDecodingError(CalldataCPEDecodingError),
    // Unexpected error.
    UnexpectedError,
}
