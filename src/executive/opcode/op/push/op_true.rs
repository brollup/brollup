use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_TRUE_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_TRUE` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_TRUE;

impl OP_TRUE {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Push True (0x01) to the main stack.
        let item_to_push = StackItem::new(vec![0x01]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_TRUE_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}

/// OP_1 is analogous to `OP_TRUE`.
#[allow(non_camel_case_types)]
pub type OP_1 = OP_TRUE;

/// Implement the `OpcodeEncoder` trait for `OP_TRUE`.
impl OpcodeEncoder for OP_TRUE {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x51])
    }
}
