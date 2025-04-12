use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_RETURNALL_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_RETURNALL` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_RETURNALL;

impl OP_RETURNALL {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<Vec<StackItem>, StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(vec![]);
        }

        // Get the stack items count.
        let stack_items_count = stack_holder.stack_items_count() as usize;

        // Collect remaining stack items.
        let mut items = Vec::<StackItem>::with_capacity(stack_items_count);

        // Collect remaining stack items.
        for _ in 0..stack_items_count {
            items.push(stack_holder.pop()?);
        }

        // Increment the ops counter.
        stack_holder.increment_ops(OP_RETURNALL_OPS)?;

        Ok(items)
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_RETURNALL`.
impl OpcodeEncoder for OP_RETURNALL {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x65])
    }
}
