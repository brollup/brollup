use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_RETURN_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_ENDIF` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_RETURN;

impl OP_RETURN {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<Vec<StackItem>, StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(vec![]);
        }

        // Collect remaining stack items.
        let mut items = Vec::<StackItem>::new();

        // Collect remaining stack items.
        for _ in 0..stack_holder.stack_len() {
            items.push(stack_holder.pop()?);
        }

        // Increment the ops counter.
        stack_holder.increment_ops(OP_RETURN_OPS)?;

        Ok(items)
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_RETURN`.
impl OpcodeEncoder for OP_RETURN {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x65])
    }
}
