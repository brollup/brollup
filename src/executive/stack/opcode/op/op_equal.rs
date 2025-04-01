use crate::executive::stack::stack::{StackError, StackHolder, StackItem};

/// The `OP_EQUAL` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_EQUAL;

impl OP_EQUAL {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop two items from stack.
        let item_1 = stack_holder.pop()?;
        let item_2 = stack_holder.pop()?;

        // Push 0x01 if the two items are equal, empty push otherwise.
        let item_to_push = match item_1.bytes() == item_2.bytes() {
            true => StackItem::new(vec![0x01]),
            false => StackItem::new(vec![]),
        };

        // Push the item to the stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}
