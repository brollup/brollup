use crate::executive::stack::stack::StackItem;

/// The `OP_DROP` opcode.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct OP_DROP;

impl OP_DROP {
    pub fn execute(_: StackItem) -> Option<StackItem> {
        None
    }
}
