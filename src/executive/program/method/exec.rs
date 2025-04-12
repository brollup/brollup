use super::method::ProgramMethod;
use crate::executive::{
    opcode::{
        op::{
            flow::{
                op_else::OP_ELSE, op_endif::OP_ENDIF, op_fail::OP_FAIL, op_if::OP_IF,
                op_nop::OP_NOP, op_notif::OP_NOTIF, op_returnall::OP_RETURNALL,
                op_returnerr::OP_RETURNERR, op_returnsome::OP_RETURNSOME, op_verify::OP_VERIFY,
            },
            push::{
                op_10::OP_10, op_11::OP_11, op_12::OP_12, op_13::OP_13, op_14::OP_14, op_15::OP_15,
                op_16::OP_16, op_2::OP_2, op_3::OP_3, op_4::OP_4, op_5::OP_5, op_6::OP_6,
                op_7::OP_7, op_8::OP_8, op_9::OP_9, op_false::OP_FALSE, op_true::OP_TRUE,
            },
        },
        opcode::Opcode,
    },
    stack::{stack_error::StackError, stack_holder::StackHolder, stack_item::StackItem},
};

/// A trait for executing a `ProgramMethod`.
pub trait MethodExecution {
    fn execute(
        &self,
        msg_sender: [u8; 32],
        initial_items: Vec<StackItem>,
        op_budget: u32,
        internal_ops_counter: &mut u32,
        external_ops_counter: &mut u32,
    ) -> Result<Vec<StackItem>, StackError>;
}

impl MethodExecution for ProgramMethod {
    fn execute(
        &self,
        msg_sender: [u8; 32],
        stack_items: Vec<StackItem>,
        op_budget: u32,
        internal_ops_counter: &mut u32,
        external_ops_counter: &mut u32,
    ) -> Result<Vec<StackItem>, StackError> {
        // Create a new stack holder with the initial stack items.
        let mut stack_holder = StackHolder::new_with_items(
            self.contract_id(),
            msg_sender,
            op_budget,
            internal_ops_counter,
            external_ops_counter,
            stack_items,
        )?;

        // Execute the script.
        for opcode in self.script().iter() {
            match opcode {
                // Data push opcodes.
                Opcode::OP_FALSE(_) => OP_FALSE::execute(&mut stack_holder)?,
                Opcode::OP_PUSHDATA(op_pushdata) => op_pushdata.execute(&mut stack_holder)?,
                Opcode::OP_TRUE(_) => OP_TRUE::execute(&mut stack_holder)?,
                Opcode::OP_2(_) => OP_2::execute(&mut stack_holder)?,
                Opcode::OP_3(_) => OP_3::execute(&mut stack_holder)?,
                Opcode::OP_4(_) => OP_4::execute(&mut stack_holder)?,
                Opcode::OP_5(_) => OP_5::execute(&mut stack_holder)?,
                Opcode::OP_6(_) => OP_6::execute(&mut stack_holder)?,
                Opcode::OP_7(_) => OP_7::execute(&mut stack_holder)?,
                Opcode::OP_8(_) => OP_8::execute(&mut stack_holder)?,
                Opcode::OP_9(_) => OP_9::execute(&mut stack_holder)?,
                Opcode::OP_10(_) => OP_10::execute(&mut stack_holder)?,
                Opcode::OP_11(_) => OP_11::execute(&mut stack_holder)?,
                Opcode::OP_12(_) => OP_12::execute(&mut stack_holder)?,
                Opcode::OP_13(_) => OP_13::execute(&mut stack_holder)?,
                Opcode::OP_14(_) => OP_14::execute(&mut stack_holder)?,
                Opcode::OP_15(_) => OP_15::execute(&mut stack_holder)?,
                Opcode::OP_16(_) => OP_16::execute(&mut stack_holder)?,
                // Flow control opcodes.
                Opcode::OP_NOP(_) => OP_NOP::execute(&mut stack_holder)?,
                Opcode::OP_RETURNERR(_) => {
                    // Get the error message item.
                    let error_message_item = OP_RETURNERR::execute(&mut stack_holder)?;

                    // Return the error item.
                    return Ok(vec![error_message_item]);
                }
                Opcode::OP_IF(_) => OP_IF::execute(&mut stack_holder)?,
                Opcode::OP_NOTIF(_) => OP_NOTIF::execute(&mut stack_holder)?,
                Opcode::OP_RETURNALL(_) => {
                    // Get the return items.
                    let return_items = OP_RETURNALL::execute(&mut stack_holder)?;

                    // Return the items.
                    return Ok(return_items);
                }
                Opcode::OP_RETURNSOME(_) => {
                    // Get the return items.
                    let return_items = OP_RETURNSOME::execute(&mut stack_holder)?;

                    // Return the items.
                    return Ok(return_items);
                }
                Opcode::OP_ELSE(_) => OP_ELSE::execute(&mut stack_holder)?,
                Opcode::OP_ENDIF(_) => OP_ENDIF::execute(&mut stack_holder)?,
                Opcode::OP_VERIFY(_) => OP_VERIFY::execute(&mut stack_holder)?,
                Opcode::OP_FAIL(_) => OP_FAIL::execute(&mut stack_holder)?,
                // Stack opcodes.
                _ => (),
            }
        }

        // Return empty stack items if so far not returned.
        Ok(vec![])
    }
}
