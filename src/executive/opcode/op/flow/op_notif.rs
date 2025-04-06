use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_NOTIF_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError},
};

/// The `OP_NOTIF` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_NOTIF;

impl OP_NOTIF {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Increment the ops counter.
        stack_holder.increment_ops(OP_NOTIF_OPS)?;

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_NOTIF`.
impl OpcodeEncoder for OP_NOTIF {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x64])
    }
}
