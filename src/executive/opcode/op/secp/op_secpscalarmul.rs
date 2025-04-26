use crate::executive::stack::{
    stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem,
};
use secp::MaybeScalar;

/// Multiplies two secp scalars together.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_SECPSCALARMUL;

/// The number of ops for the `OP_SECPSCALARMUL` opcode.
pub const SECPSCALARMUL_OPS: u32 = 5;

impl OP_SECPSCALARMUL {
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

        // Multiply the two scalars together.
        let multiplication = scalar_1 * scalar_2;

        // Convert the multiplication to the stack item.
        let multiplication_item = StackItem::new(multiplication.serialize().to_vec());

        // Push the multiplication back to the main stack.
        stack_holder.push(multiplication_item)?;

        // Increment the ops counter.
        stack_holder.increment_ops(SECPSCALARMUL_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_SECPSCALARMUL` opcode (0xaf).
    pub fn bytecode() -> Vec<u8> {
        vec![0xaf]
    }
}
