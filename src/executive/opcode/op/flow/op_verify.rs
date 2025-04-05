use crate::executive::{
    opcode::ops::OP_VERIFY_OPS,
    stack::{stack::StackHolder, stack_error::StackError},
};

/// The `OP_VERIFY` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_VERIFY;

impl OP_VERIFY {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop last from the main stack.
        let item = stack_holder.pop()?;

        // Check if the item is true.
        if item.bytes() != vec![0x01] {
            return Err(StackError::MandatoryVerifyError);
        }

        // Increment the ops counter.
        stack_holder.increment_ops(OP_VERIFY_OPS)?;

        Ok(())
    }
}
