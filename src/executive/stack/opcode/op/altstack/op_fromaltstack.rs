use crate::executive::stack::{
    opcode::ops::OP_FROMALTSTACK_OPS,
    stack::{StackError, StackHolder},
};

/// The `OP_FROMALTSTACK` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_FROMALTSTACK;

impl OP_FROMALTSTACK {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop the last item from the alt stack.
        let last_item = stack_holder.alt_stack_pop()?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_FROMALTSTACK_OPS)?;

        // Push the last item to the main stack.
        stack_holder.push(last_item)?;

        Ok(())
    }
}
