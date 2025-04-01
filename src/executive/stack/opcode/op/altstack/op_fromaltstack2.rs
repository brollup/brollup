use crate::executive::stack::stack::{StackError, StackHolder};

/// The `OP_FROMALTSTACK2` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_FROMALTSTACK2;

impl OP_FROMALTSTACK2 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Get the alt stack 2.
        let alt_stack_2 = stack_holder.alt_stack_2();

        // Pop the last item from alt stack 2.
        let last_item = alt_stack_2.pop()?;

        // Push the last item to the main stack.
        stack_holder.push(last_item)?;

        Ok(())
    }
}
