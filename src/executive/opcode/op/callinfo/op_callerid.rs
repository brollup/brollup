use crate::executive::stack::{
    stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem,
};

/// Push the caller id to the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_CALLERID;

/// The number of ops for the `OP_CALLERID` opcode.
pub const CALLERID_OPS: u32 = 1;

impl OP_CALLERID {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Get the caller id bytes.
        let caller_id_bytes: [u8; 32] = stack_holder.caller_id();

        // Push the item to the main stack.
        stack_holder.push(StackItem::new(caller_id_bytes.to_vec()))?;

        // Increment the ops counter.
        stack_holder.increment_ops(CALLERID_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_CALLERID` opcode (0xb9).
    pub fn bytecode() -> Vec<u8> {
        vec![0xb9]
    }
}
