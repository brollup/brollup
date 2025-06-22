use crate::executive::stack::{
    stack_error::StackError,
    stack_holder::StackHolder,
    stack_item::StackItem,
    stack_uint::{SafeConverter, StackItemUintExt, StackUint},
};

/// Pushes the allocated payment amount to the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_PAYABLESPENT;

/// The number of ops for the `OP_PAYABLESPENT` opcode.
pub const PAYABLESPENT_OPS: u32 = 1;

impl OP_PAYABLESPENT {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Get the spent payable amount.
        let spent_payable_amount_as_u32 = stack_holder.payable_spent_value();

        // Convert the spent payable amount to a `StackUint`.
        let spent_payable_amount_as_stack_uint = StackUint::from_u32(spent_payable_amount_as_u32);

        // Convert the spent payable amount to a `StackItem`.
        let spent_payable_amount_as_stack_item =
            StackItem::from_stack_uint(spent_payable_amount_as_stack_uint);

        // Push the spent payable amount to the stack.
        stack_holder.push(spent_payable_amount_as_stack_item)?;

        // Increment the ops counter.
        stack_holder.increment_ops(PAYABLESPENT_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_PAYABLESPENT` opcode (0xc1).
    pub fn bytecode() -> Vec<u8> {
        vec![0xc1]
    }
}
