use crate::executive::{
    opcode::{codec::OpcodeEncoder, ops::OP_FALSE_OPS},
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_FALSE` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_FALSE;

impl OP_FALSE {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Push False (empty stack item) to the main stack.
        let item_to_push = StackItem::new(vec![]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_FALSE_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}

/// OP_0 is analogous to `OP_FALSE`.
#[allow(non_camel_case_types)]
pub type OP_0 = OP_FALSE;

/// Implement the `OpcodeEncoder` trait for `OP_FALSE`.
impl OpcodeEncoder for OP_FALSE {
    fn encode(&self) -> Vec<u8> {
        vec![0x00]
    }
}
