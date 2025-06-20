use serde_json::{json, Value};

use crate::constructive::calldata::element::CallElement;

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
    args: Vec<CallElement>,
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
        args: Vec<CallElement>,
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
    pub fn args(&self) -> Vec<CallElement> {
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

    /// Returns the callholder object as a JSON value.
    pub fn json(&self) -> Value {
        // eachh arg as hex string
        let args = self
            .args
            .iter()
            .map(|arg| hex::encode(arg.into_stack_item().bytes()))
            .collect::<Vec<_>>();

        let value = json!({
            "caller_account_key": hex::encode(self.account_key),
            "callee_contract_id": hex::encode(self.contract_id),
            "method_index": self.method_index,
            "args": args,
            "timestamp": self.timestamp.to_string(),
            "ops_budget": self.ops_budget,
            "ops_price": self.ops_price,
        });

        // Return the value
        value
    }
}
