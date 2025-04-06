use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_11_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_11` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_11;

impl OP_11 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Push 11 (0x0b) to the main stack.
        let item_to_push = StackItem::new(vec![0x0b]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_11_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_11`.
impl OpcodeEncoder for OP_11 {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x5b])
    }
}
