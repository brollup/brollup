use crate::executive::{
    opcode::ops::OP_16_OPS,
    stack::{stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem},
};

/// Pushes number 16 (0x10) to the main stack.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_16;

impl OP_16 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Push 16 (0x10) to the main stack.
        let item_to_push = StackItem::new(vec![0x10]);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_16_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_16` opcode (0x60).
    pub fn bytecode() -> Vec<u8> {
        vec![0x60]
    }
}
