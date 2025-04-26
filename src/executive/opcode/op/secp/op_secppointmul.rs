use crate::executive::stack::{
    stack_error::StackError,
    stack_holder::StackHolder,
    stack_item::StackItem,
    stack_uint::{SafeConverter, StackItemUintExt},
};
use secp::MaybePoint;

/// Multiplies a secp point by a secp scalar.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_SECPSPOINTMUL;

/// The number of ops for the `OP_SECPSPOINTMUL` opcode.
pub const SECPSPOINTMUL_OPS: u32 = 10;

impl OP_SECPSPOINTMUL {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop scalar from the main stack.
        let scalar_item = stack_holder.pop()?;

        // Pop point from the main stack.
        let point_item = stack_holder.pop()?;

        // Convert the scalar to a secp scalar.
        let scalar = scalar_item
            .to_stack_uint()
            .ok_or(StackError::StackUintConversionError)?
            .to_secp_scalar()
            .ok_or(StackError::InvalidSecpScalar)?;

        // Convert the point to a secp point.
        let point = match MaybePoint::from_slice(point_item.bytes()) {
            Ok(point) => point,
            Err(_) => return Err(StackError::InvalidSecpPoint),
        };

        // Multiply the point by the scalar.
        let multiplication = point * scalar;

        // Convert the multiplication to the stack item.
        let multiplication_item = StackItem::new(multiplication.serialize_uncompressed().to_vec());

        // Push the multiplication back to the main stack.
        stack_holder.push(multiplication_item)?;

        // Increment the ops counter.
        stack_holder.increment_ops(SECPSPOINTMUL_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_SECPSPOINTMUL` opcode (0xb1).
    pub fn bytecode() -> Vec<u8> {
        vec![0xb1]
    }
}
