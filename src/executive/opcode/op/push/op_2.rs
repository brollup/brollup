use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_2_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_2` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_2;

impl OP_2 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Push 2 (0x02) to the main stack.
        let item_to_push = StackItem::new(vec![0x02]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_2_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_2`.
impl OpcodeEncoder for OP_2 {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x52])
    }
}
