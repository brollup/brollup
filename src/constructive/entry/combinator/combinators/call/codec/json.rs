use crate::constructive::entry::combinator::combinators::call::call::Call;
use serde_json::{json, Value};

impl Call {
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
