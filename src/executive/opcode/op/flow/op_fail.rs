use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_FAIL_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError},
};

/// The `OP_FAIL` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_FAIL;

impl OP_FAIL {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Increment the ops counter.
        stack_holder.increment_ops(OP_FAIL_OPS)?;

        // Fail the execution.
        Err(StackError::FailError)
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_FAIL`.
impl OpcodeEncoder for OP_FAIL {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x62])
    }
}
