use crate::executive::stack::{
    stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem,
};
use bitcoin::hashes::ripemd160;
use bitcoin::hashes::sha256;
use bitcoin::hashes::Hash;

/// The input is hashed twice: first with SHA-256 and then with RIPEMD-160.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_HASH160;

impl OP_HASH160 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the preimage from the main stack.
        let preimage = stack_holder.pop()?;

        // Hash the item using SHA-256.
        let sha256_hash = sha256::Hash::hash(preimage.bytes())
            .to_byte_array()
            .to_vec();

        // Hash the item using RIPEMD-160.
        let ripemd160_hash = ripemd160::Hash::hash(&sha256_hash).to_byte_array().to_vec();

        // Increment the ops counter.
        stack_holder.increment_ops(calculate_ops(preimage.len()))?;

        // Push the hash back to the main stack.
        stack_holder.push(StackItem::new(ripemd160_hash))?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_HASH160` opcode (0xa9).
    pub fn bytecode() -> Vec<u8> {
        vec![0xa9]
    }
}

const HASH160_OPS_BASE: u32 = 20;
const HASH160_OPS_MULTIPLIER: u32 = 1;
const HASH160_OPS_OUTPUT_LEN: u32 = 20;

// Calculate the number of ops for a OP_HASH160 opcode.
fn calculate_ops(preimage_len: u32) -> u32 {
    // Calculate the gap between the preimage length and the output length.
    let gap = match HASH160_OPS_OUTPUT_LEN.checked_sub(preimage_len) {
        Some(gap) => gap,
        None => 0,
    };

    // Return the number of ops.
    HASH160_OPS_BASE + (HASH160_OPS_MULTIPLIER * gap)
}
