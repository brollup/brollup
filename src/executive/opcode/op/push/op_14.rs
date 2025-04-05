use crate::executive::{
    opcode::ops::OP_14_OPS,
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_14` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_14;

impl OP_14 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Push 14 (0x0e) to the main stack.
        let item_to_push = StackItem::new(vec![0x0e]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_14_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}
