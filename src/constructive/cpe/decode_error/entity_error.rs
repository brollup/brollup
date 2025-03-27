use serde::{Deserialize, Serialize};

/// Error type for `Account` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountCPEDecodingError {
    // Failed to iterate over the bit stream to check if the account is registered.
    FailedToIterateIsRegisteredBit,
    // Failed to decode the registery index.
    FailedToDecodeRegisteryIndex,
    // Unable to locate the account key from the registery index.
    UnableToLocateAccountKeyGivenIndex(u32),
    // Unable to construct a new key to be registered.
    UnableToConstructNewKey,
    // Account key is already registered.
    AccountKeyAlreadyRegistered([u8; 32]),
}

/// Error type for `Contract` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractCPEDecodingError {
    // Failed to collect the is ranked bit.
    FailedToCollectIsRankedBit,
    // Failed to collect the rank index bits.
    FailedToCollectRankIndexBits,
    // Failed to locate the ranked contract.
    FailedToLocateContractGivenRankIndex(u8),
    // Failed to decode the registery index.
    FailedToDecodeRegisteryIndex,
    // Unable to locate the unranked contract.
    FailedToLocateContractGivenRegisteryIndex(u32),
}
