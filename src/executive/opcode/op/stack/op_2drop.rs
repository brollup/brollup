use crate::executive::{
    opcode::ops::OP_2DROP_OPS,
    stack::{stack_error::StackError, stack_holder::StackHolder},
};

/// Drops two items from the main stack.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_2DROP;

impl OP_2DROP {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop two items from the main stack.
        stack_holder.pop()?;
        stack_holder.pop()?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_2DROP_OPS)?;

        Ok(())
    }
}
