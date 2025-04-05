use crate::executive::{
    opcode::ops::OP_9_OPS,
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_9` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_9;

impl OP_9 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Push 9 (0x09) to the main stack.
        let item_to_push = StackItem::new(vec![0x09]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_9_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}
