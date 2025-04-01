use std::fmt;

/// The maximum number of items in the stack.
pub const MAX_STACK_ITEMS: u32 = 1024;

/// The maximum size of an item in the stack.
pub const MAX_STACK_ITEM_SIZE: u32 = 4095;

/// The type of items in the stack.
#[derive(Debug, Clone)]
pub struct StackItem(Vec<u8>);

impl StackItem {
    pub fn new(item: Vec<u8>) -> Self {
        Self(item)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }
}

impl fmt::Display for StackItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0))
    }
}

/// The stack newtype wrapper.
#[derive(Debug, Clone)]
pub struct Stack(pub Vec<StackItem>);

impl Stack {
    pub fn init() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn push(&mut self, item: StackItem) {
        self.0.push(item);
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        self.0.pop()
    }

    pub fn last_cloned(&self) -> Option<StackItem> {
        self.0.last().cloned()
    }
}
impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?; // Start with a newline
        for (i, item) in self.0.iter().enumerate() {
            writeln!(f, "{}. {}", i + 1, item)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StackError {
    /// The stack is empty.
    EmptyStack,
    /// The stack item is too large.
    StackItemTooLarge,
    /// The stack is too large.
    StackTooLarge,
}
