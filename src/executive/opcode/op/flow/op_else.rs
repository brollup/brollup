use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_ELSE_OPS,
    },
    stack::{
        stack::{FlowEncounter, StackHolder},
        stack_error::StackError,
    },
};

/// The `OP_ELSE` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_ELSE;

impl OP_ELSE {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop the latest flow encounter from the stack.
        let flow_encounter = stack_holder
            .pop_flow_encounter()
            .ok_or(StackError::OPElseEncounteredWithoutPrecedingIfNotif)?;

        // The earlier flow encounter must have been an OP_IF/OP_NOTIF.
        // Otherwise, return an error.
        if let FlowEncounter::Else = flow_encounter {
            return Err(StackError::OPElseEncounteredWithPrecedingAnotherOPElse);
        }

        // Push the Else flow encounter back to the flow encounters.
        stack_holder.new_flow_encounter(FlowEncounter::Else);

        // Pop the latest execution flag.
        let execution_flag = stack_holder
            .pop_execution_flag()
            .ok_or(StackError::OPElseEncounteredWithoutPrecedingExecutionFlag)?;

        // Push the reversed active execution flag.
        stack_holder.new_execution_flag(!execution_flag);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_ELSE_OPS)?;

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_ELSE`.
impl OpcodeEncoder for OP_ELSE {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x67])
    }
}
