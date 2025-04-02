#[cfg(test)]
mod stack_tests {
    use brollup::executive::stack::{
        opcode::op::{
            altstack::{op_fromaltstack::OP_FROMALTSTACK, op_toaltstack::OP_TOALTSTACK},
            op_cat::OP_CAT,
            op_equalverify::OP_EQUALVERIFY,
        },
        stack::{StackError, StackHolder, StackItem},
    };

    #[test]
    fn stack_test() -> Result<(), StackError> {
        let mut stack_holder = StackHolder::new([0; 32]);

        // Initialize main stack.

        // Push 0xdeadbeef
        let _ = stack_holder.push(StackItem::new(vec![0xde, 0xad, 0xbe, 0xef]));

        // Push 0xdead
        let _ = stack_holder.push(StackItem::new(vec![0xde, 0xad]));

        // Push 0xbeef
        let _ = stack_holder.push(StackItem::new(vec![0xbe, 0xef]));

        // Print the stack.
        println!("Stack: {}", stack_holder.stack());

        // OP_TOALTSTACK
        OP_TOALTSTACK::execute(&mut stack_holder)?;

        // Print the stack.
        println!("Stack: {}", stack_holder.stack());

        // OP_TOALTSTACK
        OP_TOALTSTACK::execute(&mut stack_holder)?;

        // Print the stack.
        println!("Stack: {}", stack_holder.stack());

        // OP_FROMALTSTACK
        OP_FROMALTSTACK::execute(&mut stack_holder)?;

        // Print the stack.
        println!("Stack: {}", stack_holder.stack());

        // OP_FROMALTSTACK
        OP_FROMALTSTACK::execute(&mut stack_holder)?;

        // Print the stack.
        println!("Stack: {}", stack_holder.stack());

        // OP_CAT
        OP_CAT::execute(&mut stack_holder)?;

        // Print the stack.
        println!("Stack: {}", stack_holder.stack());

        // OP_EQUALVERIFY
        OP_EQUALVERIFY::execute(&mut stack_holder)?;

        // Print the stack.
        println!("Stack: {}", stack_holder.stack());

        Ok(())
    }
}
