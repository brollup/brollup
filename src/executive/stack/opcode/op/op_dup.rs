use crate::executive::stack::stack::{Stack, StackError};

/// The `OP_DUP` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_DUP;

impl OP_DUP {
    pub fn execute(stack: &mut Stack) -> Result<(), StackError> {
        // Clone the last item from stack.
        let last_item = stack.last_cloned().ok_or(StackError::EmptyStack)?;

        // Push the cloned value back to stack.
        stack.push(last_item);

        Ok(())
    }
}
