use crate::executive::stack::{stack_error::StackError, stack_holder::StackHolder};

/// Pays one or more accounts the specified amounts.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_PAY;

impl OP_PAY {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Calculate the number of ops.
        let ops = calculate_ops(0);

        // Increment the ops counter.
        stack_holder.increment_ops(ops)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_PAY` opcode (0xc3).
    pub fn bytecode() -> Vec<u8> {
        vec![0xc3]
    }
}

const PAY_OPS_BASE: u32 = 10;
const PAY_OPS_MULTIPLIER: u32 = 5;

// Calculate the number of ops for a PAY opcode.
fn calculate_ops(count: u32) -> u32 {
    // Return the number of ops.
    PAY_OPS_BASE + (PAY_OPS_MULTIPLIER * count)
}
