use crate::executive::{
    opcode::ops::OP_VERIFY_OPS,
    stack::{stack_error::StackError, stack_holder::StackHolder},
};

/// Pops an item from the main stack and checks if it is true. Fails if it is not.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_VERIFY;

impl OP_VERIFY {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop last from the main stack.
        let item = stack_holder.pop()?;

        // Check if the item is true.
        if item.bytes() != vec![0x01] {
            return Err(StackError::MandatoryVerifyError);
        }

        // Increment the ops counter.
        stack_holder.increment_ops(OP_VERIFY_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_VERIFY` opcode.
    pub fn bytecode() -> Vec<u8> {
        vec![0x69]
    }
}
