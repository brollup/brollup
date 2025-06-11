use crate::executive::stack::{
    stack_error::{StackError, StackUintError},
    stack_holder::StackHolder,
    stack_uint::StackItemUintExt,
};

/// Pays one or more accounts the specified amounts.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_PAY;

/// The number of ops for the `OP_PAY` opcode.
pub const PAY_OPS: u32 = 10;

impl OP_PAY {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the amount from the stack.
        let amount_item = stack_holder.pop()?;

        // Pop the key from the stack.
        let key_item = stack_holder.pop()?;

        // Convert the amount to a `StackUint`.
        let amount_as_stack_uint =
            amount_item
                .to_stack_uint()
                .ok_or(StackError::StackUintError(
                    StackUintError::StackUintConversionError,
                ))?;

        let _amount_as_u32 = amount_as_stack_uint.as_u32();

        // Convert the key to [u8; 32]
        let _key_as_bytes_32: [u8; 32] = key_item
            .bytes()
            .try_into()
            .map_err(|_| StackError::Key32BytesConversionError)?;

        // TODO: implement the payment logic.

        // Increment the ops counter.
        stack_holder.increment_ops(PAY_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_PAY` opcode (0xc3).
    pub fn bytecode() -> Vec<u8> {
        vec![0xc3]
    }
}
