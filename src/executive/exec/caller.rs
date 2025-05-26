/// Caller can be the account key itself or another contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Caller {
    AccountKey([u8; 32]),
    ContractId([u8; 32]),
}

impl Caller {
    /// Creates a new caller from an account key.
    pub fn new_account_key(account_key: [u8; 32]) -> Self {
        Self::AccountKey(account_key)
    }

    /// Creates a new caller from a contract id.
    pub fn new_contract_id(contract_id: [u8; 32]) -> Self {
        Self::ContractId(contract_id)
    }

    /// Returns true if the caller is an account key.
    pub fn is_account(&self) -> bool {
        matches!(self, Self::AccountKey(_))
    }

    /// Returns true if the caller is a contract.
    pub fn is_contract(&self) -> bool {
        matches!(self, Self::ContractId(_))
    }

    /// Returns the caller id.
    pub fn caller_id(&self) -> [u8; 32] {
        match self {
            Self::AccountKey(account_key) => *account_key,
            Self::ContractId(contract_id) => *contract_id,
        }
    }
}
