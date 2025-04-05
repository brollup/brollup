use crate::executive::{
    opcode::ops::OP_15_OPS,
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_15` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_15;

impl OP_15 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Push 15 (0x0f) to the main stack.
        let item_to_push = StackItem::new(vec![0x0f]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_15_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}
