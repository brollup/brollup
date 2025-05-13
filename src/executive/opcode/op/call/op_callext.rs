use crate::executive::stack::{
    stack_error::StackError,
    stack_holder::StackHolder,
    stack_item::StackItem,
    stack_uint::{SafeConverter, StackItemUintExt},
};

/// Call an external contract.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_CALLEXT;

/// The number of ops for the `OP_CALLEXT` opcode.
pub const CALLEXT_OPS: u32 = 10;

impl OP_CALLEXT {
    pub fn execute(
        stack_holder: &mut StackHolder,
    ) -> Result<([u8; 32], u8, Vec<StackItem>), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(([0xff; 32], 0xff, vec![]));
        }

        // Pop the contract id and method index from the stack.
        let contract_id = stack_holder.pop()?;
        let method_index = stack_holder.pop()?;

        // Convert the contract id and method index to bytes.
        let contract_id_bytes: [u8; 32] = match contract_id.bytes().try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(StackError::InvalidContractId),
        };

        // Convert the method index to a u32.
        let method_index_as_u32: u32 = match method_index.to_stack_uint() {
            Some(value) => match value.to_u32() {
                Some(u32_value) => u32_value,
                None => return Err(StackError::InvalidMethodIndex),
            },
            None => return Err(StackError::InvalidMethodIndex),
        };

        // Convert the method index to a u8.
        let method_index_as_u8: u8 = match method_index_as_u32 {
            u32_value if u32_value > u8::MAX as u32 => return Err(StackError::InvalidMethodIndex),
            u32_value => u32_value as u8,
        };

        // Get the stack items count.
        let stack_items_count = stack_holder.stack_items_count() as usize;

        // Collect remaining stack items.
        let mut items = Vec::<StackItem>::with_capacity(stack_items_count);

        // Collect remaining stack items.
        for _ in 0..stack_items_count {
            items.push(stack_holder.pop()?);
        }

        // Increment the ops counter.
        stack_holder.increment_ops(CALLEXT_OPS)?;

        Ok((contract_id_bytes, method_index_as_u8, items))
    }

    /// Returns the bytecode for the `OP_CALLEXTERNAL` opcode (0xbe).
    pub fn bytecode() -> Vec<u8> {
        vec![0xbe]
    }
}
