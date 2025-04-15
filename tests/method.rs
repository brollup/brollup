#[cfg(test)]
mod method_tests {
    use brollup::{
        constructive::calldata::element_type::CallElementType,
        executive::{
            opcode::{
                op::{
                    flow::op_returnerr::OP_RETURNERR,
                    push::{op_2::OP_2, op_pushdata::OP_PUSHDATA, op_true::OP_TRUE},
                },
                opcode::Opcode,
            },
            program::method::{
                compiler::compiler::MethodCompiler, method::ProgramMethod, method_type::MethodType,
            },
        },
    };

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
