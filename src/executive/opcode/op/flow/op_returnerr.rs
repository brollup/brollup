use crate::executive::{
    opcode::ops::OP_RETURNERR_OPS,
    stack::{stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem},
};

/// Returns an error item from the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_RETURNERR;

impl OP_RETURNERR {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<StackItem, StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(StackItem::new(vec![]));
        }

        // Pop the error item from the stack.
        let error_item = stack_holder.pop()?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_RETURNERR_OPS)?;

        Ok(error_item)
    }

    /// Returns the bytecode for the `OP_RETURNERR` opcode (0x62).
    pub fn bytecode() -> Vec<u8> {
        vec![0x62]
    }
}
