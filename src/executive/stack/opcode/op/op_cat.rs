use crate::executive::stack::{
    opcode::ops::OP_CAT_OPS,
    stack::{StackError, StackHolder, StackItem},
};

/// The `OP_CAT` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_CAT;

impl OP_CAT {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop item one from the main stack.
        let item_1 = stack_holder.pop()?;

        // Pop item two from the main stack.
        let item_2 = stack_holder.pop()?;

        // Join the two items
        let mut joined = Vec::<u8>::with_capacity(item_1.len() as usize + item_2.len() as usize);
        joined.extend(item_2.bytes());
        joined.extend(item_1.bytes());

        // Increment the ops counter.
        stack_holder.increment_ops(OP_CAT_OPS)?;

        // Push the joined item back to the main stack.
        stack_holder.push(StackItem::new(joined))?;

        Ok(())
    }
}
