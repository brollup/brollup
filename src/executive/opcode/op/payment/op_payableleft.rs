use crate::executive::stack::{
    stack_error::StackError,
    stack_holder::StackHolder,
    stack_item::StackItem,
    stack_uint::{SafeConverter, StackItemUintExt, StackUint},
};

/// Pushes the left payment amount to the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_PAYABLELEFT;

/// The number of ops for the `OP_PAYABLELEFT` opcode.
pub const PAYABLELEFT_OPS: u32 = 1;

impl OP_PAYABLELEFT {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Get the payable left amount.
        let payable_left_amount_as_u64 = stack_holder.payable_left();

        // Convert the left payment amount to a stack uint64.
        let payable_left_amount_as_stack_uint = StackUint::from_u64(payable_left_amount_as_u64);

        // Convert the left payment amount to a stack item.
        let payable_left_amount_as_stack_item =
            StackItem::from_stack_uint(payable_left_amount_as_stack_uint);

        // Push the payable left amount to the stack.
        stack_holder.push(payable_left_amount_as_stack_item)?;

        // Increment the ops counter.
        stack_holder.increment_ops(PAYABLELEFT_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_PAYABLELEFT` opcode (0xc2).
    pub fn bytecode() -> Vec<u8> {
        vec![0xc2]
    }
}
