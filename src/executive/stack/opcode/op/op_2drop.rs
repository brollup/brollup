use crate::executive::stack::stack::{StackError, StackHolder};

/// The `OP_DROP` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_2DROP;

impl OP_2DROP {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop two items from the main stack.
        stack_holder.pop()?;
        stack_holder.pop()?;

        Ok(())
    }
}
