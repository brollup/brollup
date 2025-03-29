use crate::executive::stack::stack::StackItem;

/// The `OP_DUP` opcode.
#[derive(Debug, Clone, Copy)]
pub struct OP_DUP;

impl OP_DUP {
    pub fn execute(item: StackItem) -> Option<StackItem> {
        let mut result = item.clone();
        result.extend(item);
        Some(result)
    }
}
