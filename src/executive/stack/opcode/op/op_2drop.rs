use crate::executive::stack::stack::{Stack, StackError};

/// The `OP_DROP` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_2DROP;

impl OP_2DROP {
    pub fn execute(stack: &mut Stack) -> Result<(), StackError> {
        // Pop two items from stack.
        stack.pop().ok_or(StackError::EmptyStack)?;
        stack.pop().ok_or(StackError::EmptyStack)?;

        Ok(())
    }
}
