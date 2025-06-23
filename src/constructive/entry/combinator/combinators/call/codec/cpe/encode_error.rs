/// The error type for encoding a call as a CPE.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CPECallEncodeError {
    AccountKeyMismatch,
    ContractRankNotFoundAtContractId([u8; 32]),
    ContractRankCPEEncodeError,
    ContractMethodCountNotFoundAtContractId([u8; 32]),
    MethodIndexCPEEncodeError,
    ArgsCPEEncodeError,
    OpsBudgetCPEEncodeError,
    BaseOpsPriceMismatch,
    OpsPriceExtraInCPEEncodeError,
}
