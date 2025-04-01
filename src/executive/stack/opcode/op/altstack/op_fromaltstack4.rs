use crate::executive::stack::stack::{StackError, StackHolder};

/// The `OP_FROMALTSTACK4` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_FROMALTSTACK4;

impl OP_FROMALTSTACK4 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Get the alt stack 4.
        let alt_stack_4 = stack_holder.alt_stack_4();

        // Pop the last item from alt stack 4.
        let last_item = alt_stack_4.pop()?;

        // Push the last item to the main stack.
        stack_holder.push(last_item)?;

        Ok(())
    }
}
