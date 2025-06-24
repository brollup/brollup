use crate::constructive::valtype::val::short_val::cpe::decode::decode_error::ShortValCPEDecodingError;

/// Type alias for the contract rank.
type ContractRank = u32;

/// Enum to represent errors that can occur when decoding a `Contract` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContractCPEDecodingError {
    RankAsShortValDecodeError(ShortValCPEDecodingError),
    FailedToLocateContractGivenRank(ContractRank),
}
