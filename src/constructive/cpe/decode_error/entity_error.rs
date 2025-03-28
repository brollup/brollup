use serde::{Deserialize, Serialize};

/// Error type for `Account` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountCPEDecodingError {
    // Failed to decode the rank.
    FailedToDecodeRank,
    // Failed to locate the ranked account.
    FailedToLocateAccountGivenRank(u32),
    // Failed to iterate over 256 bits to collect key bits.
    FailedToColletKeyBits,
    // Failed to construct a new key to be registered.
    FailedToConstructKey,
    // Account key is already registered.
    AccountKeyAlreadyRegistered([u8; 32]),
}

/// Error type for `Contract` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractCPEDecodingError {
    // Failed to decode the rank.
    FailedToDecodeRank,
    // Failed to locate the ranked contract.
    FailedToLocateContractGivenRank(u32),
}
