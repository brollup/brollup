use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_IF_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError},
};

/// The `OP_IF` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_IF;

impl OP_IF {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Increment the ops counter.
        stack_holder.increment_ops(OP_IF_OPS)?;

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_IF`.
impl OpcodeEncoder for OP_IF {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x63])
    }
}
