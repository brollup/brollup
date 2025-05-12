use crate::executive::stack::{
    stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem,
};

/// Push the account key to the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_ACCOUNTKEY;

/// The number of ops for the `OP_ACCOUNTKEY` opcode.
pub const ACCOUNTKEY_OPS: u32 = 1;

impl OP_ACCOUNTKEY {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Get the account key bytes.
        let account_key_bytes: [u8; 32] = stack_holder.account_key();

        // Push the item to the main stack.
        stack_holder.push(StackItem::new(account_key_bytes.to_vec()))?;

        // Increment the ops counter.
        stack_holder.increment_ops(ACCOUNTKEY_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_ACCOUNTKEY` opcode (0xb9).
    pub fn bytecode() -> Vec<u8> {
        vec![0xb9]
    }
}
