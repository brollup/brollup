use crate::executive::{
    opcode::ops::OP_12_OPS,
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_12` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_12;

impl OP_12 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Push 12 (0x0c) to the main stack.
        let item_to_push = StackItem::new(vec![0x0c]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_12_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}
