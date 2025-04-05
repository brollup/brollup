use crate::executive::{
    opcode::ops::OP_3_OPS,
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_3` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_3;

impl OP_3 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Push 3 (0x03) to the main stack.
        let item_to_push = StackItem::new(vec![0x03]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_3_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}
