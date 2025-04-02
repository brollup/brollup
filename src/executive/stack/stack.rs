use std::{collections::HashMap, fmt};

/// The maximum number of items in the stack.
pub const MAX_STACK_ITEMS: u32 = 1024;

/// The maximum size of an item in the stack.
pub const MAX_STACK_ITEM_SIZE: u32 = 4095;

/// The minimum length of a memory/storage key.
pub const MIN_KEY_LENGTH: u32 = 1;

/// The maximum length of a memory/storage key.
pub const MAX_KEY_LENGTH: u32 = 40;

/// The minimum length of a memory/storage value.
pub const MIN_VALUE_LENGTH: u32 = 1;

/// The maximum byte size of a contract memory.
pub const MAX_CONTRACT_MEMORY_SIZE: u32 = 65_536;

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

/// The stack newtype wrapper.
#[derive(Debug, Clone)]
pub struct Stack(pub Vec<StackItem>);

impl Stack {
    /// Creates a new stack.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Returns the length of the stack.
    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    /// Pushes a stack item to the stack.
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

    /// Pops a stack item from the stack.
    pub fn pop(&mut self) -> Result<StackItem, StackError> {
        self.0.pop().ok_or(StackError::EmptyStack)
    }

    /// Returns the last item from the stack.
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
    // Contract id.
    contract_id: [u8; 32],
    // Main stack.
    main_stack: Stack,
    // Alt stack.
    alt_stack: Stack,
    // Contract memory.
    memory: HashMap<Vec<u8>, Vec<u8>>,
    // Contract memory size.
    memory_size: u32,
}

impl StackHolder {
    /// Creates a new stack holder.
    pub fn new(contract_id: [u8; 32]) -> Self {
        Self {
            contract_id,
            main_stack: Stack::new(),
            alt_stack: Stack::new(),
            memory: HashMap::new(),
            memory_size: 0,
        }
    }

    /// Returns the contract id.
    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    /// Returns the contract memory.
    pub fn memory(&self) -> &HashMap<Vec<u8>, Vec<u8>> {
        &self.memory
    }

    /// Returns the contract memory.
    pub fn memory_mut(&mut self) -> &mut HashMap<Vec<u8>, Vec<u8>> {
        &mut self.memory
    }

    /// Returns the contract's memory size.
    pub fn memory_size(&self) -> u32 {
        self.memory_size
    }

    /// Updates the contract's memory size.
    pub fn update_memory_size(&mut self, new_size: u32) {
        self.memory_size = new_size;
    }

    /// Returns the main stack.
    pub fn stack(&mut self) -> &mut Stack {
        &mut self.main_stack
    }

    /// Returns the alt stack.
    pub fn alt_stack(&mut self) -> &mut Stack {
        &mut self.alt_stack
    }

    /// Returns the length of the main stack.
    pub fn stack_len(&self) -> u32 {
        self.main_stack.len()
    }

    /// Returns the length of the alt stack.
    pub fn alt_stack_len(&self) -> u32 {
        self.alt_stack.len()
    }

    /// Pushes a stack item to the main stack.
    pub fn push(&mut self, item: StackItem) -> Result<(), StackError> {
        self.main_stack.push(item)
    }

    /// Pushes a stack item to alt stack.
    pub fn alt_stack_push(&mut self, item: StackItem) -> Result<(), StackError> {
        self.alt_stack.push(item)
    }

    /// Pop the last stack item from main stack.
    pub fn pop(&mut self) -> Result<StackItem, StackError> {
        self.main_stack.pop()
    }

    /// Pop the last stack item from alt stack.
    pub fn alt_stack_pop(&mut self) -> Result<StackItem, StackError> {
        self.alt_stack.pop()
    }

    /// Returns the last stack item from main stack.
    pub fn last_cloned(&self) -> Result<StackItem, StackError> {
        self.main_stack.last_cloned()
    }

    /// Returns the last stack item from alt stack.
    pub fn alt_stack_last_cloned(&self) -> Result<StackItem, StackError> {
        self.alt_stack.last_cloned()
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
    // Invalid memory key length.
    InvalidMemoryKeyLength(u8),
    // Invalid memory value length.
    InvalidMemoryValueLength(u8),
    // Invalid storage key length.
    InvalidStorageKeyLength(u8),
    // Invalid storage value length.
    InvalidStorageValueLength(u8),
    // Memory size limit exceeded.
    ContractMemorySizeLimitExceeded,
}
