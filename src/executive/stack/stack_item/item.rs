use std::fmt;

/// The type of items in the stack.
#[derive(Debug, Clone)]
pub struct StackItem(Vec<u8>);

impl StackItem {
    /// Creates a new stack item.
    pub fn new(item: Vec<u8>) -> Self {
        Self(item)
    }

    /// Returns the bytes of the stack item.
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the length of the stack item.   
    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }
}

impl fmt::Display for StackItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0))
    }
}
