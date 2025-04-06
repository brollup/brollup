use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_RETURNERR_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_RETURNERR` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_RETURNERR;

impl OP_RETURNERR {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<StackItem, StackError> {
        // Pop the error item from the stack.
        let error_item = stack_holder.pop()?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_RETURNERR_OPS)?;

        Ok(error_item)
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_RETURN`.
impl OpcodeEncoder for OP_RETURNERR {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x66])
    }
}
