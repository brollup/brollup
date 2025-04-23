use bitcoin::hashes::ripemd160;
use bitcoin::hashes::Hash;

use crate::executive::{
    opcode::ops::OP_RIPEMD160_OPS,
    stack::{stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem},
};

/// The input is hashed using RIPEMD-160.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_RIPEMD160;

impl OP_RIPEMD160 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the preimage from the main stack.
        let preimage = stack_holder.pop()?;

        // Hash the item using RIPEMD-160.
        let hash = ripemd160::Hash::hash(preimage.bytes()).to_byte_array().to_vec();

        // Push the hash back to the main stack.
        stack_holder.push(StackItem::new(hash))?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_RIPEMD160_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_RIPEMD160` opcode (0xa6).
    pub fn bytecode() -> Vec<u8> {
        vec![0xa6]
    }
}
