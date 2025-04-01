use crate::executive::stack::stack::{StackError, StackHolder, StackItem};

/// The `OP_CAT` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_CAT;

impl OP_CAT {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop item one from stack.
        let item_1 = stack_holder.pop()?;

        // Pop item two from stack.
        let item_2 = stack_holder.pop()?;

        // Join the two items
        let mut joined = Vec::<u8>::with_capacity(item_1.len() as usize + item_2.len() as usize);
        joined.extend(item_2.bytes());
        joined.extend(item_1.bytes());

        // Push the joined item back to stack.
        stack_holder.push(StackItem::new(joined))?;

        Ok(())
    }
}
