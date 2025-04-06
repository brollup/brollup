use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_NOP_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError},
};

/// The `OP_NOP` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_NOP;

impl OP_NOP {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Increment the ops counter.
        stack_holder.increment_ops(OP_NOP_OPS)?;

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_NOP`.
impl OpcodeEncoder for OP_NOP {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x61])
    }
}
