use super::exec_error::ExecutionError;
use crate::executive::{
    opcode::{
        op::push::{op_false::OP_FALSE, op_true::OP_TRUE},
        opcode::Opcode,
    },
    program::{
        method::{method::ProgramMethod, method_type::MethodType},
        program::Program,
    },
    stack::{stack_holder::StackHolder, stack_item::StackItem},
};

pub fn execute<'a>(
    // Caller can be the account key itself or another contract.
    caller_id: [u8; 32],
    // The contract id of the called contract.
    contract_id: [u8; 32],
    // The method index of the called contract.
    method_index: u8,
    // The stack items to be passed as arguments to the called contract.
    arg_values: Vec<StackItem>,
    // The timestamp.
    timestamp: u64,
    // The ops budget.
    ops_budget: u32,
    // The ops price.
    ops_price: u32,
    // The internal ops counter.
    internal_ops_counter: &'a mut u32,
    // The external ops counter.
    external_ops_counter: &'a mut u32,
) -> Result<Vec<StackItem>, ExecutionError> {
    let program = {
        // Placeholder method #1
        let method_1 = ProgramMethod::new(
            "method_1".to_string(),
            MethodType::Callable,
            vec![],
            vec![Opcode::OP_TRUE(OP_TRUE)],
        )
        .unwrap();

        // Placeholder method #2
        let method_2 = ProgramMethod::new(
            "method_2".to_string(),
            MethodType::Callable,
            vec![],
            vec![Opcode::OP_FALSE(OP_FALSE)],
        )
        .unwrap();

        // Placeholder program
        let program = Program::new("program".to_string(), vec![method_1, method_2]).unwrap();
        program
    };

    // Get the program method by index.
    let program_method = match program.method_by_index(method_index) {
        Some(method) => method,
        None => return Err(ExecutionError::MethodNotFoundAtIndexError(method_index)),
    };

    // Create a new stack holder.
    let mut stack_holder = match StackHolder::new_with_items(
        caller_id,
        contract_id,
        timestamp,
        ops_budget,
        ops_price,
        internal_ops_counter,
        external_ops_counter,
        arg_values,
    ) {
        Ok(stack_holder) => stack_holder,
        Err(error) => return Err(ExecutionError::StackHolderInitializationError(error)),
    };

    // Execute the program method.
    for opcode in program_method.script().iter() {
        match opcode {
            Opcode::OP_TRUE(OP_TRUE) => {
                OP_TRUE::execute(&mut stack_holder)
                    .map_err(|error| ExecutionError::OpcodeExecutionError(error))?;
            }
            _ => {}
        }
    }

    return Err(ExecutionError::MethodNotFoundAtIndexError(method_index));
}
