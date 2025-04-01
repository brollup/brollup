use crate::executive::stack::stack::{StackError, StackHolder};

/// The `OP_DROP` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_DROP;

impl OP_DROP {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop the last item from stack.
        stack_holder.pop()?;

        Ok(())
    }
}
