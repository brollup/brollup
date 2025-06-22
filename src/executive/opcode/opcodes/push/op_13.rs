use crate::executive::{
    opcode::ops::OP_13_OPS,
    stack::{stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem},
};

/// Pushes number 13 (0x0d) to the main stack.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_13;

impl OP_13 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Push 13 (0x0d) to the main stack.
        let item_to_push = StackItem::new(vec![0x0d]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_13_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_13` opcode (0x5d).
    pub fn bytecode() -> Vec<u8> {
        vec![0x5d]
    }
}
