use crate::executive::{
    opcode::ops::OP_MUL_OPS,
    stack::{
        stack::StackHolder,
        stack_error::StackError,
        stack_item::{item::StackItem, uint_ext::StackItemUintExt},
    },
};

/// The `OP_ADD` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_MUL;

impl OP_MUL {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop two items from the main stack.
        let item_1 = stack_holder.pop()?;
        let item_2 = stack_holder.pop()?;

        // Irem 1 uint value;
        let item_1_uint = item_1
            .to_uint()
            .ok_or(StackError::StackUintMaxOverflowError)?;

        // Item 2 uint value;
        let item_2_uint = item_2
            .to_uint()
            .ok_or(StackError::StackUintMaxOverflowError)?;

        // Multiply the two values.
        let result = match item_1_uint.checked_mul(item_2_uint) {
            // If the result is an overflow, return False (an empty stack item).
            None => StackItem::new(vec![]),
            // If the result is not an overflow, return the result.
            Some(result) => StackItem::from_uint(result),
        };

        // Increment the ops counter.
        stack_holder.increment_ops(OP_MUL_OPS)?;

        // Push the result to the main stack.
        stack_holder.push(result)?;

        Ok(())
    }
}
