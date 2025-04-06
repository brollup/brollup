use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_ELSE_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError},
};

/// The `OP_ELSE` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_ELSE;

impl OP_ELSE {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Increment the ops counter.
        stack_holder.increment_ops(OP_ELSE_OPS)?;

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_ELSE`.
impl OpcodeEncoder for OP_ELSE {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x67])
    }
}
