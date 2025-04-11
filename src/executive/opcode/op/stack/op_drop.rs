use crate::executive::{
    opcode::ops::OP_DROP_OPS,
    stack::{stack::StackHolder, stack_error::StackError},
};

/// The `OP_DROP` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_DROP;

impl OP_DROP {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the last stack item from the main stack.
        stack_holder.pop()?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_DROP_OPS)?;

        Ok(())
    }
}
