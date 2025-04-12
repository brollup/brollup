use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_VERIFY_OPS,
    },
    stack::{stack_error::StackError, stack_holder::StackHolder},
};

/// Pops an item from the main stack and checks if it is true. Fails if it is not.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_VERIFY;

impl OP_VERIFY {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

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
/// Implement the `OpcodeEncoder` trait for `OP_VERIFY`.
impl OpcodeEncoder for OP_VERIFY {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x69])
    }
}
