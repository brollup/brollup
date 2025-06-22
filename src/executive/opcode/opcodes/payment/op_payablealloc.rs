use crate::executive::stack::{
    stack_error::StackError,
    stack_holder::StackHolder,
    stack_item::StackItem,
    stack_uint::{SafeConverter, StackItemUintExt, StackUint},
};

/// Pushes the allocated payment amount to the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_PAYABLEALLOC;

/// The number of ops for the `OP_PAYABLEALLOC` opcode.
pub const PAYABLEALLOC_OPS: u32 = 1;

impl OP_PAYABLEALLOC {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Get the allocated payable amount.
        let allocated_payable_amount_as_u32 = stack_holder.payable_allocation_value();

        // Convert the allocated payable amount to a `StackUint`.
        let allocated_payable_amount_as_stack_uint =
            StackUint::from_u32(allocated_payable_amount_as_u32);

        // Convert the allocated payable amount to a `StackItem`.
        let allocated_payable_amount_as_stack_item =
            StackItem::from_stack_uint(allocated_payable_amount_as_stack_uint);

        // Push the allocated payable amount to the stack.
        stack_holder.push(allocated_payable_amount_as_stack_item)?;

        // Increment the ops counter.
        stack_holder.increment_ops(PAYABLEALLOC_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_PAYABLEALLOC` opcode (0xc0).
    pub fn bytecode() -> Vec<u8> {
        vec![0xc0]
    }
}
