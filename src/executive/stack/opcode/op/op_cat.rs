use crate::executive::stack::stack::{
    Stack, StackError, StackItem, MAX_STACK_ITEMS, MAX_STACK_ITEM_SIZE,
};

/// The `OP_CAT` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_CAT;

impl OP_CAT {
    pub fn execute(stack: &mut Stack) -> Result<(), StackError> {
        // Pop item one from stack.
        let item_1 = stack.pop().ok_or(StackError::EmptyStack)?;

        // Pop item two from stack.
        let item_2 = stack.pop().ok_or(StackError::EmptyStack)?;

        // Check the combined value lengths.
        let joined_length = match item_1.len().checked_add(item_2.len()) {
            Some(len) if len <= MAX_STACK_ITEM_SIZE => len,
            _ => return Err(StackError::StackItemTooLarge),
        };

        // Join the two items
        let mut joined = Vec::<u8>::with_capacity(joined_length as usize);
        joined.extend(item_2.bytes());
        joined.extend(item_1.bytes());

        // Check the stack size.
        if stack.len() > MAX_STACK_ITEMS {
            return Err(StackError::StackTooLarge);
        }

        // Push the joined item back to stack.
        stack.push(StackItem::new(joined));

        Ok(())
    }
}
