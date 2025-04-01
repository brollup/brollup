use crate::executive::stack::stack::{StackError, StackHolder};

/// The `OP_TOALTSTACK4` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_TOALTSTACK4;

impl OP_TOALTSTACK4 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop the last item from stack.
        let last_item = stack_holder.pop()?;

        // Get the alt stack 4.
        let alt_stack = stack_holder.alt_stack_4();

        // Push the last item to the alt stack.
        alt_stack.push(last_item)?;

        Ok(())
    }
}
