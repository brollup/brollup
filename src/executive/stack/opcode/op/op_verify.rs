use crate::executive::stack::{
    opcode::ops::OP_VERIFY_OPS,
    stack::{StackError, StackHolder},
};

/// The `OP_EQUALVERIFY` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_EQUALVERIFY;

impl OP_EQUALVERIFY {
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
