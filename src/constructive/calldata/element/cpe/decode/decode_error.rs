use crate::constructive::{
    entity::{
        account::cpe::decode::decode_error::AccountCPEDecodingError,
        contract::cpe::decode::decode_error::ContractCPEDecodingError,
    },
    valtype::maybe_common::maybe_common::cpe::decode::decode_error::MaybeCommonCPEDecodingError,
};

/// Type alias for the bytes length.
type BytesLength = usize;

/// Type alias for the varbytes byte length.
type VarbytesByteLength = u16;

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum U8ArgCPEDecodingError {
    Collect8BitsError,
    ConvertToByteError,
}

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum U16ArgCPEDecodingError {
    Collect16BitsError,
    ConvertToBytesError,
}

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum U32ArgCPEDecodingError {
    MaybeCommonShortValCPEDecodingError(MaybeCommonCPEDecodingError),
}

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum U64ArgCPEDecodingError {
    MaybeCommonLongValCPEDecodingError(MaybeCommonCPEDecodingError),
}

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BoolArgCPEDecodingError {
    CollectBoolBitError,
}

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountArgCPEDecodingError {
    AccountCPEDecodingError(AccountCPEDecodingError),
}

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContractArgCPEDecodingError {
    ContractCPEDecodingError(ContractCPEDecodingError),
}

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BytesArgCPEDecodingError {
    InvalidBytesLength(BytesLength),
    CollectDataBitsError,
}

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VarbytesArgCPEDecodingError {
    CollectVarbytesLengthBitsError,
    ByteLengthGreaterThan4095Error(VarbytesByteLength),
    CollectVarbytesDataBitsError,
}

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PayableArgCPEDecodingError {
    MaybeCommonShortValCPEDecodingError(MaybeCommonCPEDecodingError),
}

/// Enum to represent errors that can occur when decoding a `CallElement` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CallArgCPEDecodingError {
    U8(U8ArgCPEDecodingError),
    U16(U16ArgCPEDecodingError),
    U32(U32ArgCPEDecodingError),
    U64(U64ArgCPEDecodingError),
    Bool(BoolArgCPEDecodingError),
    Account(AccountArgCPEDecodingError),
    Contract(ContractArgCPEDecodingError),
    Bytes(BytesArgCPEDecodingError),
    Varbytes(VarbytesArgCPEDecodingError),
    Payable(PayableArgCPEDecodingError),
}
