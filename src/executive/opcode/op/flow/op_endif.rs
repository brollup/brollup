use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_ENDIF_OPS,
    },
    stack::{stack::StackHolder, stack_error::StackError},
};

/// The `OP_ENDIF` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_ENDIF;

impl OP_ENDIF {
    pub fn execute(stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // Pop the latest execution flag from the stack.
        let _flow_encounter = stack_holder
            .pop_flow_encounter()
            .ok_or(StackError::OPElseEncounteredWithoutPrecedingFlowEncounter)?;

        // Increment the ops counter.
        stack_holder.increment_ops(OP_ENDIF_OPS)?;

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_ENDIF`.
impl OpcodeEncoder for OP_ENDIF {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        Ok(vec![0x68])
    }
}
