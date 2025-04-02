use crate::executive::stack::stack::{StackError, StackHolder};

/// The `OP_DUP` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_DUP;

impl OP_DUP {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Clone the last stack item from the main stack.
        let last_item = stack_holder.last_cloned()?;

        // Push the cloned value back to the main stack.
        stack_holder.push(last_item)?;

        Ok(())
    }
}
