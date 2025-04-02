use crate::executive::stack::stack::{StackError, StackHolder};

/// The `OP_FROMALTSTACK` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_FROMALTSTACK;

impl OP_FROMALTSTACK {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop the last item from the alt stack.
        let last_item = stack_holder.alt_stack_pop()?;

        // Push the last item to the main stack.
        stack_holder.push(last_item)?;

        Ok(())
    }
}
