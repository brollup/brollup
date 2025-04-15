#[cfg(test)]
mod method_tests {
    use brollup::{
        constructive::calldata::element_type::CallElementType,
        executive::{
            opcode::{
                op::{
                    flow::op_returnall::OP_RETURNALL,
                    flow::op_returnerr::OP_RETURNERR,
                    push::{op_2::OP_2, op_pushdata::OP_PUSHDATA, op_true::OP_TRUE},
                    reserved::op_reserved1::OP_RESERVED_1,
                },
                opcode::Opcode,
            },
            program::method::{
                compiler::compiler::MethodCompiler, method::ProgramMethod, method_type::MethodType,
            },
        },
    };

    #[test]
    fn method_construction_test() -> Result<(), String> {
        // Invalid name length.
        {
            let method_name = "a".to_string();
            let method_type = MethodType::Callable;
            let call_element_types = vec![CallElementType::U32, CallElementType::Account];
            let script = vec![
                Opcode::OP_TRUE(OP_TRUE),
                Opcode::OP_2(OP_2),
                Opcode::OP_PUSHDATA(OP_PUSHDATA(vec![0xde, 0xad, 0xbe, 0xef])),
                Opcode::OP_RETURNERR(OP_RETURNERR),
            ];

            let method = ProgramMethod::new(method_name, method_type, call_element_types, script);

            assert!(method.is_err());
        }

        // Invalid call element type count.
        {
            let method_name = "test_method".to_string();
            let method_type = MethodType::Callable;
            // Push more than the maximum (16) allowed call element types; 17
            let call_element_types = vec![
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
                CallElementType::U32,
            ];
            let script = vec![
                Opcode::OP_TRUE(OP_TRUE),
                Opcode::OP_2(OP_2),
                Opcode::OP_PUSHDATA(OP_PUSHDATA(vec![0xde, 0xad, 0xbe, 0xef])),
                Opcode::OP_RETURNERR(OP_RETURNERR),
            ];

            let method = ProgramMethod::new(method_name, method_type, call_element_types, script);

            assert!(method.is_err());
        }

        // Invalid opcode count.
        {
            let method_name = "test_method".to_string();
            let method_type = MethodType::Callable;
            let call_element_types = vec![CallElementType::U32, CallElementType::Account];
            // Push less than the minimum (4) allowed opcodes; 3
            let script = vec![
                Opcode::OP_TRUE(OP_TRUE),
                Opcode::OP_2(OP_2),
                Opcode::OP_PUSHDATA(OP_PUSHDATA(vec![0xde, 0xad, 0xbe, 0xef])),
            ];

            let method = ProgramMethod::new(method_name, method_type, call_element_types, script);

            assert!(method.is_err());
        }

        // Invalid script with reserved opcode.
        {
            let method_name = "test_method".to_string();
            let method_type = MethodType::Callable;
            let call_element_types = vec![CallElementType::U32, CallElementType::Account];
            // Push a reserved opcode.
            let script = vec![
                Opcode::OP_TRUE(OP_TRUE),
                Opcode::OP_2(OP_2),
                Opcode::OP_PUSHDATA(OP_PUSHDATA(vec![0xde, 0xad, 0xbe, 0xef])),
                Opcode::OP_RESERVED_1(OP_RESERVED_1),
            ];

            let method = ProgramMethod::new(method_name, method_type, call_element_types, script);

            assert!(method.is_err());
        }

        // Invalid script with non minimal push data.
        {
            let method_name = "test_method".to_string();
            let method_type = MethodType::Callable;
            let call_element_types = vec![CallElementType::U32, CallElementType::Account];
            // Push a non minimal push data.
            let script = vec![
                Opcode::OP_TRUE(OP_TRUE),
                Opcode::OP_2(OP_2),
                Opcode::OP_PUSHDATA(OP_PUSHDATA(vec![0x05])), // Should have been OP_5.
                Opcode::OP_RETURNALL(OP_RETURNALL),
            ];

            let method = ProgramMethod::new(method_name, method_type, call_element_types, script);

            assert!(method.is_err());
        }

        // Valid construction.
        {
            let method_name = "test_method".to_string();
            let method_type = MethodType::Callable;
            let call_element_types = vec![CallElementType::U32, CallElementType::Account];
            let script = vec![
                Opcode::OP_TRUE(OP_TRUE),
                Opcode::OP_2(OP_2),
                Opcode::OP_PUSHDATA(OP_PUSHDATA(vec![0xde, 0xad, 0xbe, 0xef])),
                Opcode::OP_RETURNALL(OP_RETURNALL),
            ];

            let method = ProgramMethod::new(method_name, method_type, call_element_types, script);

            assert!(method.is_ok());
        }
        Ok(())
    }

    #[test]
    fn method_compiler_test() -> Result<(), String> {
        let method_name = "test_method".to_string();
        let method_type = MethodType::Callable;
        let call_element_types = vec![CallElementType::U32, CallElementType::Account];
        let script = vec![
            Opcode::OP_TRUE(OP_TRUE),
            Opcode::OP_2(OP_2),
            Opcode::OP_PUSHDATA(OP_PUSHDATA(vec![0xde, 0xad, 0xbe, 0xef])),
            Opcode::OP_RETURNERR(OP_RETURNERR),
        ];

        let method =
            ProgramMethod::new(method_name, method_type, call_element_types, script).unwrap();

        let mut method_compiled_bytestream = method.compile().unwrap().into_iter();

        let method_decompiled = ProgramMethod::decompile(&mut method_compiled_bytestream).unwrap();

        assert_eq!(method, method_decompiled);

        Ok(())
    }
}
