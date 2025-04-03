use crate::executive::stack::{
    opcode::ops::OP_DROP_OPS,
    stack::{StackError, StackHolder},
};

/// The `OP_DROP` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_DROP;

impl OP_DROP {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop the last stack item from the main stack.
        stack_holder.pop()?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_DROP_OPS)?;

        Ok(())
    }
}
