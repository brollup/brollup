use crate::constructive::calldata::element::CallElement;
use crate::constructive::entity::account::Account;
use crate::constructive::entry::combinator::combinator_type::CombinatorType;
use crate::transmutative::hash::Hash;
use crate::transmutative::{hash::HashTag, secp::authenticable::AuthSighash};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// The holder of a call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Call {
    /// The account key.
    account_key: [u8; 32],
    /// The contract id.
    contract_id: [u8; 32],
    /// The method index.
    method_index: u8,
    /// The arguments.
    args: Vec<CallElement>,
    /// The ops budget.
    ops_budget: u32,
    /// The base ops price.
    base_ops_price: u32,
    /// The timestamp.
    timestamp: u64,
}

impl Call {
    /// Creates a new call holder.
    pub fn new(
        account_key: [u8; 32],
        contract_id: [u8; 32],
        method_index: u8,
        args: Vec<CallElement>,
        ops_budget: u32,
        base_ops_price: u32,
        timestamp: u64,
    ) -> Self {
        Self {
            account_key,
            contract_id,
            method_index,
            args,
            ops_budget,
            base_ops_price,
            timestamp,
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

    /// Returns the ops budget.
    pub fn ops_budget(&self) -> u32 {
        self.ops_budget
    }

    /// Returns the base ops price.
    pub fn base_ops_price(&self) -> u32 {
        self.base_ops_price
    }

    /// Returns the timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Validates the account for the call.
    ///
    /// This function checks if the account key matches the account key in the call.
    pub fn validate_account(&self, account: Account) -> bool {
        self.account_key == account.key().serialize_xonly()
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
            "ops_budget": self.ops_budget,
            "timestamp": self.timestamp.to_string(),
        });

        // Return the value
        value
    }
}

impl AuthSighash for Call {
    fn auth_sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        // Account key
        preimage.extend(self.account_key);

        // Contract id
        preimage.extend(self.contract_id);

        // Method index as u32
        preimage.extend((self.method_index as u32).to_le_bytes());

        // Number of args as u32
        preimage.extend((self.args.len() as u32).to_le_bytes());

        // Args
        for arg in self.args.iter() {
            preimage.extend(arg.into_stack_item().bytes());
        }

        // Ops budget as u32
        preimage.extend((self.ops_budget as u32).to_le_bytes());

        // Base ops price as u32
        preimage.extend((self.base_ops_price as u32).to_le_bytes());

        // Timestamp as u64
        preimage.extend(&self.timestamp.to_le_bytes());

        // Hash the preimage
        preimage.hash(Some(HashTag::SighashCombinator(CombinatorType::Call)))
    }
}
