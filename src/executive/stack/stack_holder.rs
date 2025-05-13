use super::{
    flow::{flow_encounter::FlowEncounter, flow_status::FlowStatus},
    limits::OPS_LIMIT,
    stack::Stack,
    stack_error::StackError,
    stack_item::StackItem,
};
use std::collections::HashMap;

/// The stack holder.
#[derive(Debug)]
pub struct StackHolder<'a> {
    // Caller id (can be an account key or a contract id).
    caller_id: [u8; 32],
    // Contract id of the contract being executed.
    contract_id: [u8; 32],
    // Timestamp.
    timestamp: u64,
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
    // Ops price.
    ops_price: u32,
    // Internal ops counter.
    internal_ops_counter: &'a mut u32,
    // External ops counter.
    external_ops_counter: &'a mut u32,
    // List of flow encounters nested in each other.
    // Since OP_IF/OP_NOTIF/OP_ELSE/OP_ENDIF can be nested, we need to keep track of the flow encounters.
    flow_encounters: Vec<FlowEncounter>,
}

impl<'a> StackHolder<'a> {
    /// Creates a new stack holder.
    pub fn new(
        caller_id: [u8; 32],
        contract_id: [u8; 32],
        timestamp: u64,
        ops_budget: u32,
        ops_price: u32,
        internal_ops_counter: &'a mut u32,
        external_ops_counter: &'a mut u32,
    ) -> Result<Self, StackError> {
        // Check if the internal ops counter exceeds the ops budget.
        if *internal_ops_counter > ops_budget {
            return Err(StackError::InternalOpsBudgetExceeded);
        }

        // Check if the external ops counter exceeds the limit.
        if *external_ops_counter > OPS_LIMIT {
            return Err(StackError::ExternalOpsLimitExceeded);
        }

        // Create a new stack holder.
        let stack_holder = Self {
            caller_id,
            contract_id,
            timestamp,
            main_stack: Stack::new(),
            alt_stack: Stack::new(),
            memory: HashMap::new(),
            memory_size: 0,
            ops_budget,
            ops_price,
            internal_ops_counter,
            external_ops_counter,
            flow_encounters: Vec::<FlowEncounter>::new(),
        };

        // Return the stack holder.
        Ok(stack_holder)
    }

    /// Creates a new stack holder and initializes it with the given items.
    pub fn new_with_items<'b>(
        caller_id: [u8; 32],
        contract_id: [u8; 32],
        timestamp: u64,
        ops_budget: u32,
        ops_price: u32,
        internal_ops_counter: &'b mut u32,
        external_ops_counter: &'b mut u32,
        initial_stack_items: Vec<StackItem>,
    ) -> Result<StackHolder<'b>, StackError>
    where
        'b: 'a,
        'a: 'b,
    {
        // Create a new stack holder.
        let mut stack_holder = Self::new(
            caller_id,
            contract_id,
            timestamp,
            ops_budget,
            ops_price,
            internal_ops_counter,
            external_ops_counter,
        )?;

        // Push the items to the stack.
        for item in initial_stack_items {
            stack_holder.push(item)?;
        }

        // Return the stack holder.
        Ok(stack_holder)
    }

    /// Returns the contract id.
    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    /// Returns the caller id.
    pub fn caller_id(&self) -> [u8; 32] {
        self.caller_id
    }

    /// Returns the timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Returns the ops budget.
    pub fn ops_budget(&self) -> u32 {
        self.ops_budget
    }

    /// Returns the ops price.
    pub fn ops_price(&self) -> u32 {
        self.ops_price
    }

    /// Returns the internal ops counter.
    pub fn internal_ops_counter(&self) -> u32 {
        *self.internal_ops_counter
    }

    /// Returns the external ops counter.
    pub fn external_ops_counter(&self) -> u32 {
        *self.external_ops_counter
    }

    pub fn increment_ops(&mut self, ops: u32) -> Result<(), StackError> {
        let new_internal_ops_counter = (*self.internal_ops_counter)
            .checked_add(ops)
            .ok_or(StackError::InternalOpsBudgetExceeded)?;

        if new_internal_ops_counter > self.ops_budget {
            return Err(StackError::InternalOpsBudgetExceeded);
        }

        let new_external_ops_counter = (*self.external_ops_counter)
            .checked_add(ops)
            .ok_or(StackError::ExternalOpsLimitExceeded)?;

        if new_external_ops_counter > OPS_LIMIT {
            return Err(StackError::ExternalOpsLimitExceeded);
        }

        *self.internal_ops_counter = new_internal_ops_counter;
        *self.external_ops_counter = new_external_ops_counter;

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

    /// Returns the items count of the main stack.
    pub fn stack_items_count(&self) -> u32 {
        self.main_stack.items_count()
    }

    /// Returns the items count of the alt stack.
    pub fn alt_stack_items_count(&self) -> u32 {
        self.alt_stack.items_count()
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

    /// Clones and returns the last stack item from main stack.
    pub fn last_item(&self) -> Result<StackItem, StackError> {
        self.main_stack.last_item()
    }

    /// Clones and returns the last stack item from alt stack.
    pub fn alt_stack_last_item(&self) -> Result<StackItem, StackError> {
        self.alt_stack.last_item()
    }

    /// Clones and returns the stack item by depth.
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
