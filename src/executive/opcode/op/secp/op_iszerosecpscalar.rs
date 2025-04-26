use crate::executive::stack::{
    stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem,
};
use secp::MaybeScalar;

/// Checks if a secp scalar is zero.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_ISZEROSECPSCALAR;

/// The number of ops for the `OP_ISZEROSECPSCALAR` opcode.
pub const ISZEROSECPSCALAR_OPS: u32 = 1;

impl OP_ISZEROSECPSCALAR {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Clone the scalar from the main stack.
        let scalar_item = stack_holder.last_item()?;

        // Convert the scalar to a secp scalar.
        let scalar = match MaybeScalar::from_slice(scalar_item.bytes()) {
            Ok(scalar) => scalar,
            Err(_) => return Err(StackError::InvalidSecpScalarBytes),
        };

        // Check if the scalar is zero.
        let result_item = match scalar.is_zero() {
            true => StackItem::true_item(),
            false => StackItem::false_item(),
        };

        // Push the result back to the main stack.
        stack_holder.push(result_item)?;

        // Increment the ops counter.
        stack_holder.increment_ops(ISZEROSECPSCALAR_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_ISZEROSECPSCALAR` opcode (0xb2).
    pub fn bytecode() -> Vec<u8> {
        vec![0xb2]
    }
}
