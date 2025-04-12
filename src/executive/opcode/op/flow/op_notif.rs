use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_NOTIF_OPS,
    },
    stack::{
        flow::{flow_encounter::FlowEncounter, flow_status::FlowStatus},
        stack_error::StackError,
        stack_holder::StackHolder,
    },
};

/// The `OP_NOTIF` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_NOTIF;

impl OP_NOTIF {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Increment the ops counter.
        stack_holder.increment_ops(OP_NOTIF_OPS)?;

        // If this is not the active execution, return.
        if !stack_holder.active_execution() {
            stack_holder.push_flow_encounter(FlowEncounter::IfNotif(FlowStatus::Uncovered));
            return Ok(());
        }

        // Pop the latest item from the stack.
        let item = stack_holder.pop()?;

        // If the item is not false, set the active execution flag to false.
        if !item.is_false() {
            stack_holder.push_flow_encounter(FlowEncounter::IfNotif(FlowStatus::Inactive));
        } else {
            stack_holder.push_flow_encounter(FlowEncounter::IfNotif(FlowStatus::Active));
        }

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_NOTIF`.
impl OpcodeEncoder for OP_NOTIF {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x64])
    }
}
