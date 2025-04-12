use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_FAIL_OPS,
    },
    stack::{stack_error::StackError, stack_holder::StackHolder},
};

/// Fails the execution.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_FAIL;

impl OP_FAIL {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Increment the ops counter.
        stack_holder.increment_ops(OP_FAIL_OPS)?;

        // Fail the execution.
        Err(StackError::FailError)
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_FAIL`.
impl OpcodeEncoder for OP_FAIL {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x6a])
    }
}
