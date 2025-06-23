/// Types for account key.
type ExpectedAccountKey = [u8; 32];
type FoundAccountKey = [u8; 32];

/// Types for ops price.
type ExpectedBaseOpsPrice = u32;
type FoundBaseOpsPrice = u32;

/// The error type for encoding a call as a CPE.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CallCPEEncodeError {
    AccountKeyMismatch(ExpectedAccountKey, FoundAccountKey),
    ContractRankNotFoundAtContractId([u8; 32]),
    ContractRankCPEEncodeError,
    ContractMethodCountNotFoundAtContractId([u8; 32]),
    MethodIndexCPEEncodeError,
    ArgsCPEEncodeError,
    OpsBudgetCPEEncodeError,
    BaseOpsPriceMismatch(ExpectedBaseOpsPrice, FoundBaseOpsPrice),
    OpsPriceExtraInCPEEncodeError,
}
