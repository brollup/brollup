use crate::executive::{
    opcode::ops::OP_TOALTSTACK_OPS,
    stack::{stack_error::StackError, stack_holder::StackHolder},
};

/// Pops an item from the main stack and pushes it to the alt stack.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_TOALTSTACK;

impl OP_TOALTSTACK {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the last item from the main stack.
        let last_item = stack_holder.pop()?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_TOALTSTACK_OPS)?;

        // Push the last item to the alt stack.
        stack_holder.alt_stack_push(last_item)?;

        Ok(())
    }
}
