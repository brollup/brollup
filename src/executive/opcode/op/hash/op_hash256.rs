use crate::executive::stack::{
    stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem,
};
use bitcoin::hashes::sha256;
use bitcoin::hashes::Hash;

/// The input is hashed two times with SHA-256.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_HASH256;

impl OP_HASH256 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the preimage from the main stack.
        let preimage = stack_holder.pop()?;

        // Hash the item using SHA-256.
        let first_hash = sha256::Hash::hash(preimage.bytes())
            .to_byte_array()
            .to_vec();

        // Hash the item, again, using SHA-256.
        let second_hash = sha256::Hash::hash(&first_hash).to_byte_array().to_vec();

        // Increment the ops counter.
        stack_holder.increment_ops(calculate_ops(preimage.len()))?;

        // Push the hash back to the main stack.
        stack_holder.push(StackItem::new(second_hash))?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_HASH256` opcode (0xaa).
    pub fn bytecode() -> Vec<u8> {
        vec![0xaa]
    }
}

const HASH256_OPS_BASE: u32 = 20;
const HASH256_OPS_MULTIPLIER: u32 = 1;
const HASH256_OPS_OUTPUT_LEN: u32 = 32;

// Calculate the number of ops for a OP_HASH256 opcode.
fn calculate_ops(preimage_len: u32) -> u32 {
    // Calculate the gap between the preimage length and the output length.
    let gap = match HASH256_OPS_OUTPUT_LEN.checked_sub(preimage_len) {
        Some(gap) => gap,
        None => 0,
    };

    // Return the number of ops.
    HASH256_OPS_BASE + (HASH256_OPS_MULTIPLIER * gap)
}
