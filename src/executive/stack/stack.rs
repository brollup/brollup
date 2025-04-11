use super::{stack_error::StackError, stack_item::item::StackItem};
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

// Ops upper bound.
pub const OPS_LIMIT: u32 = 100_000;

/// The stack newtype wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack(pub Vec<StackItem>);

impl Stack {
    /// Creates a new stack.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_with_items(items: Vec<StackItem>) -> Self {
        Self(items)
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
    pub fn last_item(&self) -> Result<StackItem, StackError> {
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

/// The status of the flow encounter.
#[derive(Debug, Clone, PartialEq)]
pub enum FlowStatus {
    Active,
    Inactive,
    Uncovered,
}

/// Tells whether the current execution is in an `if_notif`/`else` block.
#[derive(Debug, Clone)]
pub enum FlowEncounter {
    IfNotif(FlowStatus),
    Else(FlowStatus),
}

#[derive(Debug, Clone)]
pub struct StackHolder {
    // Contract id.
    contract_id: [u8; 32],
    // Msg sender.
    msg_sender: [u8; 32],
    // Main stack.
    main_stack: Stack,
    // Alt stack.
    alt_stack: Stack,
    // Contract memory.
    memory: HashMap<Vec<u8>, Vec<u8>>,
    // Contract memory size.
    memory_size: u32,
    // Ops budget.
    ops_budget: u32,
    // Internal ops counter.
    internal_ops_counter: u32,
    // External ops counter.
    external_ops_counter: u32,
    // List of flow encounters nested in each other.
    // Since OP_IF/OP_NOTIF/OP_ELSE/OP_ENDIF can be nested, we need to keep track of the flow encounters.
    flow_encounters: Vec<FlowEncounter>,
}

impl StackHolder {
    /// Creates a new stack holder.
    pub fn new(
        contract_id: [u8; 32],
        msg_sender: [u8; 32],
        ops_budget: u32,
        external_ops_counter: u32,
    ) -> Self {
        Self {
            contract_id,
            msg_sender,
            main_stack: Stack::new(),
            alt_stack: Stack::new(),
            memory: HashMap::new(),
            memory_size: 0,
            ops_budget,
            internal_ops_counter: 0,
            external_ops_counter,
            flow_encounters: Vec::<FlowEncounter>::new(),
        }
    }

    /// Returns the contract id.
    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    /// Returns the msg sender.
    pub fn msg_sender(&self) -> [u8; 32] {
        self.msg_sender
    }

    /// Returns the ops budget.
    pub fn ops_budget(&self) -> u32 {
        self.ops_budget
    }

    /// Returns the internal ops counter.
    pub fn internal_ops_counter(&self) -> u32 {
        self.internal_ops_counter
    }

    /// Returns the external ops counter.
    pub fn external_ops_counter(&self) -> u32 {
        self.external_ops_counter
    }

    pub fn increment_ops(&mut self, ops: u32) -> Result<(), StackError> {
        let new_internal_ops_counter = self.internal_ops_counter + ops;

        if new_internal_ops_counter > self.ops_budget {
            return Err(StackError::InternalOpsBudgetExceeded);
        }

        let new_external_ops_counter = self.external_ops_counter + ops;

        if new_external_ops_counter > OPS_LIMIT {
            return Err(StackError::ExternalOpsLimitExceeded);
        }

        self.internal_ops_counter = new_internal_ops_counter;
        self.external_ops_counter = new_external_ops_counter;

        Ok(())
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
    pub fn last_item(&self) -> Result<StackItem, StackError> {
        self.main_stack.last_item()
    }

    /// Returns the last stack item from alt stack.
    pub fn alt_stack_last_item(&self) -> Result<StackItem, StackError> {
        self.alt_stack.last_item()
    }

    /// Returns the stack item by depth.
    pub fn item_by_depth(&self, depth: u32) -> Result<StackItem, StackError> {
        self.main_stack
            .0
            .get(depth as usize)
            .cloned()
            .ok_or(StackError::PickIndexError(depth))
    }

    /// Removes the stack item by depth.
    pub fn remove_item_by_depth(&mut self, depth: u32) -> Result<(), StackError> {
        if depth as usize >= self.main_stack.0.len() {
            return Err(StackError::RemoveIndexError(depth));
        }
        self.main_stack.0.remove(depth as usize);
        Ok(())
    }

    /// Pushes a new flow encounter.
    pub fn push_flow_encounter(&mut self, encounter: FlowEncounter) {
        self.flow_encounters.push(encounter);
    }

    /// Pops the latest flow encounter.
    pub fn pop_flow_encounter(&mut self) -> Option<FlowEncounter> {
        self.flow_encounters.pop()
    }

    /// Returns the number of flow encounter left.
    /// We expect it to be zero end of any valid execution.
    pub fn flow_encounters_len(&self) -> usize {
        self.flow_encounters.len()
    }

    /// Returns whether the current opcode being encountered is meant to be executed.
    pub fn active_execution(&self) -> bool {
        // If there are no flow encounters, the execution is active.
        if self.flow_encounters.is_empty() {
            return true;
        }

        // Check if all flow encounters are active.
        self.flow_encounters
            .iter()
            .all(|encounter| match encounter {
                FlowEncounter::IfNotif(status) => status == &FlowStatus::Active,
                FlowEncounter::Else(status) => status == &FlowStatus::Active,
            })
    }
}
