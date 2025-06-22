use crate::executive::stack::{stack_error::StackError, stack_holder::StackHolder};

/// Fails the execution.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_RESERVED_5;

impl OP_RESERVED_5 {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Fail the execution.
        Err(StackError::ReservedOpcodeEncounteredError)
    }

    /// Returns the bytecode for the `OP_RESERVED_5` opcode (0x97).
    pub fn bytecode() -> Vec<u8> {
        vec![0x97]
    }
}
