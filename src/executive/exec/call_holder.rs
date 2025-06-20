use crate::constructive::calldata::element_type::CallElementType;

/// The holder of a call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallHolder {
    /// The account key.
    account_key: [u8; 32],
    /// The contract id.
    contract_id: [u8; 32],
    /// The method index.
    method_index: u8,
    /// The arguments.
    args: Vec<CallElementType>,
    /// The timestamp.
    timestamp: u64,
    /// The ops budget.
    ops_budget: u32,
    /// The ops price.
    ops_price: u32,
}

impl CallHolder {
    /// Creates a new call holder.
    pub fn new(
        account_key: [u8; 32],
        contract_id: [u8; 32],
        method_index: u8,
        args: Vec<CallElementType>,
        timestamp: u64,
        ops_budget: u32,
        ops_price: u32,
    ) -> Self {
        Self {
            account_key,
            contract_id,
            method_index,
            args,
            timestamp,
            ops_budget,
            ops_price,
        }
    }

    /// Returns the account key.
    pub fn account_key(&self) -> [u8; 32] {
        self.account_key
    }

    /// Returns the contract id.
    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    /// Returns the method index.
    pub fn method_index(&self) -> u8 {
        self.method_index
    }

    /// Returns the arguments.
    pub fn args(&self) -> Vec<CallElementType> {
        self.args.clone()
    }

    /// Returns the timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Returns the ops budget.
    pub fn ops_budget(&self) -> u32 {
        self.ops_budget
    }

    /// Returns the ops price.
    pub fn ops_price(&self) -> u32 {
        self.ops_price
    }
}
