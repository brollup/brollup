use crate::executive::stack::{stack::StackHolder, stack_error::StackError};

/// The `OP_PICK` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_PICK;

impl OP_PICK {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop the last item from stack.
        let _last_item = stack_holder.pop()?;

        Ok(())
    }
}
