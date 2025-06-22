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
    ops_price_base: u32,
    /// The extra ops price.
    ops_price_extra_in: Option<u32>,
}

impl Call {
    /// Creates a new call holder.
    pub fn new(
        account_key: [u8; 32],
        contract_id: [u8; 32],
        method_index: u8,
        args: Vec<CallElement>,
        ops_budget: u32,
        ops_price_base: u32,
        ops_price_extra_in: Option<u32>,
    ) -> Self {
        Self {
            account_key,
            contract_id,
            method_index,
            args,
            ops_budget,
            ops_price_base,
            ops_price_extra_in,
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
    pub fn ops_price_base(&self) -> u32 {
        self.ops_price_base
    }

    /// Returns the extra ops price.
    pub fn ops_price_extra_in(&self) -> Option<u32> {
        self.ops_price_extra_in
    }

    /// Returns the total ops price.
    pub fn ops_price_total(&self) -> u32 {
        self.ops_price_base + self.ops_price_extra_in.unwrap_or(0)
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
            "ops_price_base": self.ops_price_base,
            "ops_price_extra_in": self.ops_price_extra_in,
            "ops_price_total": self.ops_price_total(),
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
        preimage.extend((self.ops_price_base as u32).to_le_bytes());

        // Extra ops price.
        match self.ops_price_extra_in {
            Some(extra_ops_price) => {
                preimage.push(0x01);
                preimage.extend((extra_ops_price as u32).to_le_bytes());
            }
            None => {
                preimage.push(0x00);
            }
        }

        // Hash the preimage
        preimage.hash(Some(HashTag::SighashCombinator(CombinatorType::Call)))
    }
}
