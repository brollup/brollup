use crate::executive::{
    opcode::ops::OP_PICK_OPS,
    stack::{stack::StackHolder, stack_error::StackError, stack_item::uint_ext::StackItemUintExt},
};

/// The `OP_PICK` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_PICK;

impl OP_PICK {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the last item from stack.
        let last_item = stack_holder.pop()?;

        // Get the pick depth from the last item.
        let pick_depth = last_item.to_uint().ok_or(StackError::StackUintMaxOverflowError)?;

        // Get the item at the pick depth.
        let item = stack_holder.item_by_depth(pick_depth.as_u32())?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_PICK_OPS)?;

        // Push the item onto the stack.
        stack_holder.push(item)?;

        Ok(())
    }
}
