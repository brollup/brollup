use crate::executive::stack::{
    stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem,
};
use secp::MaybeScalar;

/// Adds two secp scalars together.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_SECPSCALARADD;

/// The number of ops for the `OP_SECPSCALARADD` opcode.
pub const SECPSCALARADD_OPS: u32 = 3;

impl OP_SECPSCALARADD {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop scalar 1 from the main stack.
        let scalar_1_item = stack_holder.pop()?;

        // Pop scalar 2 from the main stack.
        let scalar_2_item = stack_holder.pop()?;

        // Convert the first scalar to a secp scalar.
        let scalar_1 = match MaybeScalar::from_slice(scalar_1_item.bytes()) {
            Ok(scalar) => scalar,
            Err(_) => return Err(StackError::InvalidSecpScalarBytes),
        };

        // Convert the second scalar to a secp scalar.
        let scalar_2 = match MaybeScalar::from_slice(scalar_2_item.bytes()) {
            Ok(scalar) => scalar,
            Err(_) => return Err(StackError::InvalidSecpScalarBytes),
        };

        // Add the two scalars together.
        let addition = scalar_1 + scalar_2;

        // Convert the addition to the stack item.
        let addition_item = StackItem::new(addition.serialize().to_vec());

        // Push the addition back to the main stack.
        stack_holder.push(addition_item)?;

        // Increment the ops counter.
        stack_holder.increment_ops(SECPSCALARADD_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_SECPSCALARADD` opcode (0xae).
    pub fn bytecode() -> Vec<u8> {
        vec![0xae]
    }
}
