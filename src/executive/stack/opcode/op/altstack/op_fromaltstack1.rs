use crate::executive::stack::stack::{StackError, StackHolder};

/// The `OP_FROMALTSTACK1` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_FROMALTSTACK1;

impl OP_FROMALTSTACK1 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Get the alt stack 1.
        let alt_stack_1 = stack_holder.alt_stack_1();

        // Pop the last item from alt stack 1.
        let last_item = alt_stack_1.pop()?;

        // Push the last item to the main stack.
        stack_holder.push(last_item)?;

        Ok(())
    }
}
