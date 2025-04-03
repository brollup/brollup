#[cfg(test)]
mod stack_tests {
    use brollup::executive::stack::{
        opcode::op::{
            altstack::{op_fromaltstack::OP_FROMALTSTACK, op_toaltstack::OP_TOALTSTACK},
            op_cat::OP_CAT,
            op_equalverify::OP_EQUALVERIFY,
        },
        stack::{StackError, StackHolder, StackItem},
        stack_int::{StackInt, U256},
    };

    #[test]
    fn stack_test() -> Result<(), StackError> {
        let mut stack_holder = StackHolder::new([0; 32], 14, 0);

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

    #[test]
    #[deny(overflowing_literals)]
    fn stack_int_test() -> Result<(), StackError> {
        // Test 0
        let stack_item: StackItem = StackInt::from_u256(U256::from(0));
        assert_eq!(stack_item.to_u256(), U256::from(0));
        assert_eq!(stack_item.bytes().len(), 0);
        // Test 1
        let stack_item: StackItem = StackInt::from_u256(U256::from(1));
        assert_eq!(stack_item.to_u256(), U256::from(1));
        assert_eq!(stack_item.bytes().len(), 1);
        // Test 2
        let stack_item: StackItem = StackInt::from_u256(U256::from(2));
        assert_eq!(stack_item.to_u256(), U256::from(2));
        assert_eq!(stack_item.bytes().len(), 1);
        // Test 3
        let stack_item: StackItem = StackInt::from_u256(U256::from(3));
        assert_eq!(stack_item.to_u256(), U256::from(3));
        assert_eq!(stack_item.bytes().len(), 1);
        // Test 10
        let stack_item: StackItem = StackInt::from_u256(U256::from(10));
        assert_eq!(stack_item.to_u256(), U256::from(10));
        assert_eq!(stack_item.bytes().len(), 1);
        // Test 100
        let stack_item: StackItem = StackInt::from_u256(U256::from(100));
        assert_eq!(stack_item.to_u256(), U256::from(100));
        assert_eq!(stack_item.bytes().len(), 1);
        // Test 255
        let stack_item: StackItem = StackInt::from_u256(U256::from(255));
        assert_eq!(stack_item.to_u256(), U256::from(255));
        assert_eq!(stack_item.bytes().len(), 1);
        // Test 256 (now we are in the 2-byte range)
        let stack_item: StackItem = StackInt::from_u256(U256::from(256));
        assert_eq!(stack_item.to_u256(), U256::from(256));
        assert_eq!(stack_item.bytes().len(), 2);
        // Test 1_000
        let stack_item: StackItem = StackInt::from_u256(U256::from(1000));
        assert_eq!(stack_item.to_u256(), U256::from(1000));
        assert_eq!(stack_item.bytes().len(), 2);
        // Test 10_000
        let stack_item: StackItem = StackInt::from_u256(U256::from(10000));
        assert_eq!(stack_item.to_u256(), U256::from(10000));
        assert_eq!(stack_item.bytes().len(), 2);
        // Test 65535
        let stack_item: StackItem = StackInt::from_u256(U256::from(65535));
        assert_eq!(stack_item.to_u256(), U256::from(65535));
        assert_eq!(stack_item.bytes().len(), 2);
        // Test 65536 (now we are in the 3-byte range)
        let stack_item: StackItem = StackInt::from_u256(U256::from(65536));
        assert_eq!(stack_item.to_u256(), U256::from(65536));
        assert_eq!(stack_item.bytes().len(), 3);
        // Test 100_000
        let stack_item: StackItem = StackInt::from_u256(U256::from(100000));
        assert_eq!(stack_item.to_u256(), U256::from(100000));
        assert_eq!(stack_item.bytes().len(), 3);
        // Test 1_000_000
        let stack_item: StackItem = StackInt::from_u256(U256::from(1000000));
        assert_eq!(stack_item.to_u256(), U256::from(1000000));
        assert_eq!(stack_item.bytes().len(), 3);
        // Test 16777215
        let stack_item: StackItem = StackInt::from_u256(U256::from(16777215));
        assert_eq!(stack_item.to_u256(), U256::from(16777215));
        assert_eq!(stack_item.bytes().len(), 3);
        // Test 16777216 (now we are in the 4-byte range)
        let stack_item: StackItem = StackInt::from_u256(U256::from(16777216));
        assert_eq!(stack_item.to_u256(), U256::from(16777216));
        assert_eq!(stack_item.bytes().len(), 4);
        // Test 1_000_000_000
        let stack_item: StackItem = StackInt::from_u256(U256::from(1000000000));
        assert_eq!(stack_item.to_u256(), U256::from(1000000000));
        assert_eq!(stack_item.bytes().len(), 4);
        // Test 4,294,967,295
        let stack_item: StackItem = StackInt::from_u256(U256::from(4_294_967_295_u64));
        assert_eq!(stack_item.to_u256(), U256::from(4_294_967_295_u64));
        assert_eq!(stack_item.bytes().len(), 4);
        // Test 4,294,967,296 (now we are in the 5-byte range)
        let stack_item: StackItem = StackInt::from_u256(U256::from(4_294_967_296_u64));
        assert_eq!(stack_item.to_u256(), U256::from(4_294_967_296_u64));
        assert_eq!(stack_item.bytes().len(), 5);
        // Test 1_000_000_000_000
        let stack_item: StackItem = StackInt::from_u256(U256::from(1000000000000_i64));
        assert_eq!(stack_item.to_u256(), U256::from(1000000000000_i64));
        assert_eq!(stack_item.bytes().len(), 5);
        // Test 1_099_511_627_775
        let stack_item: StackItem = StackInt::from_u256(U256::from(1099511627775_i64));
        assert_eq!(stack_item.to_u256(), U256::from(1099511627775_i64));
        assert_eq!(stack_item.bytes().len(), 5);
        // Test 1_099_511_627_776 (now we are in the 6-byte range)
        let stack_item: StackItem = StackInt::from_u256(U256::from(1099511627776_i64));
        assert_eq!(stack_item.to_u256(), U256::from(1099511627776_i64));
        assert_eq!(stack_item.bytes().len(), 6);
        // Test 281474976710655
        let stack_item: StackItem = StackInt::from_u256(U256::from(281474976710655_i64));
        assert_eq!(stack_item.to_u256(), U256::from(281474976710655_i64));
        assert_eq!(stack_item.bytes().len(), 6);
        // Test 281474976710656 (now we are in the 7-byte range)
        let stack_item: StackItem = StackInt::from_u256(U256::from(281474976710656_i64));
        assert_eq!(stack_item.to_u256(), U256::from(281474976710656_i64));
        assert_eq!(stack_item.bytes().len(), 7);
        // Test 72057594037927935
        let stack_item: StackItem = StackInt::from_u256(U256::from(72057594037927935_i64));
        assert_eq!(stack_item.to_u256(), U256::from(72057594037927935_i64));
        assert_eq!(stack_item.bytes().len(), 7);
        // Test 72057594037927936 (now we are in the 8-byte range)
        let stack_item: StackItem = StackInt::from_u256(U256::from(72057594037927936_i64));
        assert_eq!(stack_item.to_u256(), U256::from(72057594037927936_i64));
        assert_eq!(stack_item.bytes().len(), 8);
        // Test 18446744073709551615
        let stack_item: StackItem = StackInt::from_u256(U256::from(18446744073709551615_u64));
        assert_eq!(stack_item.to_u256(), U256::from(18446744073709551615_u64));
        assert_eq!(stack_item.bytes().len(), 8);
        // Test 18446744073709551616 (now we are in the 9-byte range)
        let stack_item: StackItem = StackInt::from_u256(U256::from(18446744073709551616_u128));
        assert_eq!(stack_item.to_u256(), U256::from(18446744073709551616_u128));
        assert_eq!(stack_item.bytes().len(), 9);

        Ok(())
    }
}
