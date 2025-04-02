use crate::executive::stack::stack::{StackError, StackHolder};

/// The `OP_TOALTSTACK` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_TOALTSTACK;

impl OP_TOALTSTACK {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop the last item from the main stack.
        let last_item = stack_holder.pop()?;

        // Push the last item to the alt stack.
        stack_holder.alt_stack_push(last_item)?;

        Ok(())
    }
}
