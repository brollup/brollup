use crate::executive::{
    opcode::ops::OP_LESSTHAN_OPS,
    stack::{
        stack_error::{StackError, StackUintError},
        stack_holder::StackHolder,
        stack_item::StackItem,
        stack_uint::StackItemUintExt,
    },
};

/// Returns 1 if a is less than b, 0 otherwise.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_LESSTHAN;

impl OP_LESSTHAN {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the first item from the main stack.
        let item_b = stack_holder.pop()?;

        // Pop the second item from the main stack.
        let item_a = stack_holder.pop()?;

        // Convert item 1 to a stack uint.
        let num_b = item_b.to_stack_uint().ok_or(StackError::StackUintError(
            StackUintError::StackUintConversionError,
        ))?;

        // Convert item 2 to a stack uint.
        let num_a = item_a.to_stack_uint().ok_or(StackError::StackUintError(
            StackUintError::StackUintConversionError,
        ))?;

        // Push 1 if a is less than b, 0 otherwise.
        match num_a < num_b {
            true => stack_holder.push(StackItem::true_item())?,
            false => stack_holder.push(StackItem::false_item())?,
        }

        // Increment the ops counter.
        stack_holder.increment_ops(OP_LESSTHAN_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_LESSTHAN` opcode (0x9f).
    pub fn bytecode() -> Vec<u8> {
        vec![0x9f]
    }
}
