use crate::executive::{
    opcode::ops::OP_SHA1_OPS,
    stack::{stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem},
};
use bitcoin::hashes::sha1;
use bitcoin::hashes::Hash;

/// The input is hashed using SHA-1.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_SHA1;

impl OP_SHA1 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the preimage from the main stack.
        let preimage = stack_holder.pop()?;

        // Hash the item using SHA-1.
        let hash = sha1::Hash::hash(preimage.bytes()).to_byte_array().to_vec();

        // Push the hash back to the main stack.
        stack_holder.push(StackItem::new(hash))?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_SHA1_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_SHA1` opcode (0xa7).
    pub fn bytecode() -> Vec<u8> {
        vec![0xa7]
    }
}
