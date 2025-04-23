use crate::{
    executive::{
        opcode::ops::OP_TAGGEDHASH_OPS,
        stack::{stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem},
    },
    transmutive::hash::{Hash, HashTag},
};

/// The input is hashed with a domain seperation tag.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_TAGGEDHASH;

impl OP_TAGGEDHASH {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the tag from the main stack.
        let tag = stack_holder.pop()?;

        // Pop the preimage from the main stack.
        let preimage = stack_holder.pop()?;

        // Hash the item with the given tag.
        let hash = match tag.is_true() {
            // The tag is non-empty.
            true => preimage
                .bytes()
                .hash(Some(HashTag::CustomBytes(tag.bytes().to_vec()))),
            // The tag is empty.
            false => preimage.bytes().hash(None),
        };

        // Push the hash back to the main stack.
        stack_holder.push(StackItem::new(hash.to_vec()))?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_TAGGEDHASH_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_TAGGEDHASH` opcode (0xab).
    pub fn bytecode() -> Vec<u8> {
        vec![0xab]
    }
}
