#[cfg(test)]
mod stack_tests {
    use brollup::executive::stack::{
        opcode::op::{op_2drop::OP_2DROP, op_cat::OP_CAT, op_drop::OP_DROP, op_dup::OP_DUP},
        stack::{Stack, StackError, StackItem},
    };

    #[test]
    fn stack_test() -> Result<(), StackError> {
        let mut stack = Stack::init();

        // Push 0xaa
        stack.push(StackItem::new(vec![0xaa]));

        // Push 0xdead
        stack.push(StackItem::new(vec![0xde, 0xad]));

        // Push 0xdeadbeef
        stack.push(StackItem::new(vec![0xde, 0xad, 0xbe, 0xef]));

        // Print the stack.
        println!("Stack: {}", stack);

        // OP_DROP
        OP_DROP::execute(&mut stack)?;

        // Print the stack.
        println!("Stack: {}", stack);

        // OP_CAT
        OP_CAT::execute(&mut stack)?;

        // Print the stack.
        println!("Stack: {}", stack);

        // OP_DUP
        OP_DUP::execute(&mut stack)?;

        // Print the stack.
        println!("Stack: {}", stack);

        // OP_2DROP
        OP_2DROP::execute(&mut stack)?;

        // Print the stack.
        println!("Stack: {}", stack);

        // Assert the stack is empty.
        assert_eq!(stack.len(), 0);

        Ok(())
    }
}
