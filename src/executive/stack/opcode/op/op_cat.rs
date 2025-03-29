use crate::executive::stack::stack::StackItem;

/// The `OP_CAT` opcode.
#[derive(Debug, Clone, Copy)]
pub struct OP_CAT;

impl OP_CAT {
    pub fn execute(item_1: StackItem, item_2: StackItem) -> Option<StackItem> {
        let mut result = item_1;
        result.extend(item_2);
        Some(result)
    }
}
