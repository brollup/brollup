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
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn push(&mut self, item: StackItem) -> Result<(), StackError> {
        if item.len() > MAX_STACK_ITEM_SIZE {
            return Err(StackError::StackItemTooLarge);
        }

        if self.len() >= MAX_STACK_ITEMS {
            return Err(StackError::StackTooLarge);
        }
        self.0.push(item);

        Ok(())
    }

    pub fn pop(&mut self) -> Result<StackItem, StackError> {
        self.0.pop().ok_or(StackError::EmptyStack)
    }

    pub fn last_cloned(&self) -> Result<StackItem, StackError> {
        self.0.last().cloned().ok_or(StackError::EmptyStack)
    }
}
impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for (i, item) in self.0.iter().enumerate() {
            writeln!(f, "{}. {}", i + 1, item)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StackHolder {
    main_stack: Stack,
    alt_stack_1: Stack,
    alt_stack_2: Stack,
    alt_stack_3: Stack,
    alt_stack_4: Stack,
}

impl StackHolder {
    pub fn new() -> Self {
        Self {
            main_stack: Stack::new(),
            alt_stack_1: Stack::new(),
            alt_stack_2: Stack::new(),
            alt_stack_3: Stack::new(),
            alt_stack_4: Stack::new(),
        }
    }

    pub fn main_stack(&mut self) -> &mut Stack {
        &mut self.main_stack
    }

    pub fn alt_stack_1(&mut self) -> &mut Stack {
        &mut self.alt_stack_1
    }

    pub fn alt_stack_2(&mut self) -> &mut Stack {
        &mut self.alt_stack_2
    }

    pub fn alt_stack_3(&mut self) -> &mut Stack {
        &mut self.alt_stack_3
    }

    pub fn alt_stack_4(&mut self) -> &mut Stack {
        &mut self.alt_stack_4
    }

    pub fn len(&self) -> u32 {
        self.main_stack.len()
    }

    pub fn push(&mut self, item: StackItem) -> Result<(), StackError> {
        self.main_stack.push(item)
    }

    pub fn pop(&mut self) -> Result<StackItem, StackError> {
        self.main_stack.pop()
    }

    pub fn last_cloned(&self) -> Result<StackItem, StackError> {
        self.main_stack.last_cloned()
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
    // Equalverify error.
    EqualVerifyError,
}
