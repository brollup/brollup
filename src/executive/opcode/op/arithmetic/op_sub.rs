use crate::executive::{
    opcode::ops::OP_SUB_OPS,
    stack::{
        stack::StackHolder,
        stack_error::StackError,
        stack_item::{item::StackItem, uint_ext::StackItemUintExt},
    },
};

/// The `OP_SUB` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_SUB;

impl OP_SUB {
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

        // Increment the ops counter.
        stack_holder.increment_ops(OP_SUB_OPS)?;

        // Subtract the two values.
        match item_1_uint.checked_sub(item_2_uint) {
            // If the result is an overflow, return False (an empty stack item).
            None => {
                // Push old value to the main stack.
                stack_holder.push(item_1)?;

                // Push old value to the main stack.
                stack_holder.push(item_2)?;

                // Push False (an empty stack item) to the main stack.
                stack_holder.push(StackItem::new(vec![]))?;
            }
            // If the result is not an overflow, return the result.
            Some(result) => {
                // Push the result to the main stack.
                stack_holder.push(StackItem::from_uint(result))?;

                // Push True to the main stack.
                stack_holder.push(StackItem::new(vec![0x01]))?;
            }
        };

        Ok(())
    }
}
