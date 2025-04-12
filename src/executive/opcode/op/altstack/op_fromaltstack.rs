use crate::executive::{
    opcode::ops::OP_FROMALTSTACK_OPS,
    stack::{stack_error::StackError, stack_holder::StackHolder},
};

/// Pops an item from the alt stack and pushes it to the main stack.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_FROMALTSTACK;

impl OP_FROMALTSTACK {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the last item from the alt stack.
        let last_item = stack_holder.alt_stack_pop()?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_FROMALTSTACK_OPS)?;

        // Push the last item to the main stack.
        stack_holder.push(last_item)?;

        Ok(())
    }
}
