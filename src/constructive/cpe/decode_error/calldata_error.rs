use super::{
    entity_error::{AccountCPEDecodingError, ContractCPEDecodingError},
    valtype_error::MaybeCommonCPEDecodingError,
};
use serde::{Deserialize, Serialize};

/// Error type for `Calldata` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalldataCPEDecodingError {
    U8DecodingError,
    U16DecodingError,
    U32DecodingError(MaybeCommonCPEDecodingError),
    U64DecodingError(MaybeCommonCPEDecodingError),
    BoolDecodingError,
    AccountDecodingError(AccountCPEDecodingError),
    ContractDecodingError(ContractCPEDecodingError),
    BytesDecodingError(BytesDecodingError),
    VarbytesDecodingError(VarbytesDecodingError),
}

/// Error type for `Bytes` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BytesDecodingError {
    InvalidBytesLength,
    UnableToCollectBytesDataBits,
}

/// Error type for `Varbytes` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VarbytesDecodingError {
    UnableToCollectVarbytesLengthBits,
    VarbytesLengthGreaterThan4095,
    UnableToCollectVarbytesDataBits,
}
