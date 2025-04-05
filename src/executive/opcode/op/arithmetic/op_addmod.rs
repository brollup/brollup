use crate::executive::{
    opcode::ops::OP_ADDMOD_OPS,
    stack::{
        stack::StackHolder,
        stack_error::StackError,
        stack_item::{
            item::StackItem,
            uint_ext::{StackItemUintExt, StackUint},
        },
    },
};

/// The `OP_ADDMOD` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_ADDMOD;

impl OP_ADDMOD {
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

        // Add the two values modulo MAX::U256.
        let result = StackItem::from_uint(StackUint::addmod(&item_1_uint, &item_2_uint));

        // Increment the ops counter.
        stack_holder.increment_ops(OP_ADDMOD_OPS)?;

        // Push the result to the main stack.
        stack_holder.push(result)?;

        Ok(())
    }
}
