use crate::executive::stack::{
    opcode::ops::OP_EQUALVERIFY_OPS,
    stack::{StackError, StackHolder},
};

/// The `OP_EQUALVERIFY` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_EQUALVERIFY;

impl OP_EQUALVERIFY {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop two items from the main stack.
        let item_1 = stack_holder.pop()?;
        let item_2 = stack_holder.pop()?;

        // Check if the two items are equal.
        if item_1.bytes() != item_2.bytes() {
            return Err(StackError::MandatoryEqualVerifyError);
        }

        // Increment the ops counter.
        stack_holder.increment_ops(OP_EQUALVERIFY_OPS)?;

        Ok(())
    }
}
