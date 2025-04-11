use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_NOTIF_OPS,
    },
    stack::{
        stack::{FlowEncounter, StackHolder},
        stack_error::StackError,
    },
};

/// The `OP_NOTIF` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_NOTIF;

impl OP_NOTIF {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Pop the latest item from the stack.
        let item = stack_holder.pop()?;

        // If the item is not false, set the active execution flag to false.
        // The proceeding opcodes will not be executed.
        if !item.is_false() {
            stack_holder.new_execution_flag(false);
        } else {
            stack_holder.new_execution_flag(true);
        }

        // Push the latest flow encounter to the stack.
        stack_holder.new_flow_encounter(FlowEncounter::IfNotif);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_NOTIF_OPS)?;

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_NOTIF`.
impl OpcodeEncoder for OP_NOTIF {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x64])
    }
}
