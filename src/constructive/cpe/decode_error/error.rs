use super::{
    calldata_error::CalldataCPEDecodingError,
    entity_error::{AccountCPEDecodingError, ContractCPEDecodingError},
    entry_error::LiftupCPEDecodingError,
    valtype_error::{
        AtomicValCPEDecodingError, LongValCPEDecodingError, MaybeCommonCPEDecodingError,
        ShortValCPEDecodingError,
    },
};
use async_trait::async_trait;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// Trait for encoding structs for compact Bitcoin-DA storage.
#[async_trait]
pub trait CompactPayloadEncoding {
    /// Encode the struct into a bitvec.
    fn encode_cpe(&self) -> BitVec;
}

/// Compact payload decoding is implemented individually for each struct that implements `CompactPayloadEncoding`, rather than using a trait.
/// Refer to the CPE decoding error types listed below:

/// /// Error type for compact payload decoding.
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
