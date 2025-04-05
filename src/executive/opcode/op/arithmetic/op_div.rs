use crate::executive::{
    opcode::ops::OP_DIV_OPS,
    stack::{
        stack::StackHolder,
        stack_error::StackError,
        stack_item::{
            item::StackItem,
            uint_ext::{StackItemUintExt, StackUint},
        },
    },
};

/// The `OP_DIV` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_DIV;

impl OP_DIV {
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

        // Check if the divisor is zero.
        if item_2_uint == StackUint::zero() {
            // Push False (an empty stack item) to the main stack.
            stack_holder.push(StackItem::new(vec![]))?;
            return Ok(());
        }

        // Divide the two values.
        let (division, modulo) = item_1_uint.div_mod(item_2_uint);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_DIV_OPS)?;

        // Push the modulo result to the main stack.
        stack_holder.push(StackItem::from_uint(modulo))?;

        // Push the division result to the main stack.
        stack_holder.push(StackItem::from_uint(division))?;

        Ok(())
    }
}
