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

        // Get the allocated payment amount.
        let allocated_payment_amount_as_u64 = stack_holder.payable_allocation();

        // Convert the allocated payment amount to a stack uint64.
        let allocated_payment_amount_as_stack_uint =
            StackUint::from_u64(allocated_payment_amount_as_u64);

        // Convert the allocated payment amount to a stack item.
        let allocated_payment_amount_as_stack_item =
            StackItem::from_stack_uint(allocated_payment_amount_as_stack_uint);

        // Push the allocated payment amount to the stack.
        stack_holder.push(allocated_payment_amount_as_stack_item)?;

        // Increment the ops counter.
        stack_holder.increment_ops(PAYABLEALLOC_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_PAYABLEALLOC` opcode (0xc0).
    pub fn bytecode() -> Vec<u8> {
        vec![0xc0]
    }
}
