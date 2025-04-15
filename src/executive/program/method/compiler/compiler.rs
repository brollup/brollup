use crate::{
    constructive::calldata::element_type::CallElementType,
    executive::{
        opcode::{compiler::compiler::OpcodeCompiler, opcode::Opcode},
        program::method::{
            compiler::compiler_error::MethodCompilerError,
            limits::{
                MAX_METHOD_CALL_ELEMENT_TYPE_COUNT, MAX_METHOD_NAME_LENGTH,
                MAX_METHOD_OPCODE_COUNT, MIN_METHOD_CALL_ELEMENT_TYPE_COUNT,
                MIN_METHOD_NAME_LENGTH, MIN_METHOD_OPCODE_COUNT,
            },
            method::ProgramMethod,
            method_type::MethodType,
        },
    },
};

/// A trait for compiling and decompiling a program method.
pub trait MethodCompiler {
    /// Compiles the method into a bytecode.
    fn compile(&self) -> Result<Vec<u8>, MethodCompilerError>;
    /// Decompiles a method from a bytecode stream.
    fn decompile<I>(bytecode_stream: &mut I) -> Result<ProgramMethod, MethodCompilerError>
    where
        I: Iterator<Item = u8>;
}

impl MethodCompiler for ProgramMethod {
    fn compile(&self) -> Result<Vec<u8>, MethodCompilerError> {
        // Compile the script.
        let mut method_bytes = Vec::<u8>::new();

        // Check method name length.
        if self.method_name().len() > MAX_METHOD_NAME_LENGTH
            || self.method_name().len() < MIN_METHOD_NAME_LENGTH
        {
            return Err(MethodCompilerError::InvalidNameLength);
        }

        // Check call element type count.
        if self.call_element_types().len() > MAX_METHOD_CALL_ELEMENT_TYPE_COUNT
            || self.call_element_types().len() < MIN_METHOD_CALL_ELEMENT_TYPE_COUNT
        {
            return Err(MethodCompilerError::CallElementTypesCountError);
        }

        // Check opcode count.
        if self.script().len() > MAX_METHOD_OPCODE_COUNT
            || self.script().len() < MIN_METHOD_OPCODE_COUNT
        {
            return Err(MethodCompilerError::OpcodeCountError);
        }

        // Encode method name byte length as u8.
        method_bytes.push(self.method_name().len() as u8);

        // Encode method name.
        method_bytes.extend(self.method_name().as_bytes());

        // Encode method type.
        method_bytes.push(self.method_type().bytecode());

        // Check the number of call element types.
        if self.call_element_types().len() > u8::MAX as usize {
            return Err(MethodCompilerError::CallElementTypesCountError);
        }

        // Encode the number of call element types as u8.
        method_bytes.push(self.call_element_types().len() as u8);

        // Encode individual call element types.
        for element_type in self.call_element_types().iter() {
            method_bytes.extend(element_type.bytecode());
        }

        // Check the number of opcodes.
        if self.script().len() > MAX_METHOD_OPCODE_COUNT
            || self.script().len() < MIN_METHOD_OPCODE_COUNT
        {
            return Err(MethodCompilerError::OpcodeCountError);
        }

        // Get the number of opcodes as u16.
        let opcodes_count = self.script().len() as u16;

        // Encode the number of opcodes.
        method_bytes.extend(opcodes_count.to_le_bytes());

        // Encode individual opcodes.
        for opcode in self.script().iter() {
            method_bytes.extend(
                opcode
                    .compile()
                    .map_err(|e| MethodCompilerError::OpcodeCompileError(e))?,
            );
        }

        // Return the bytecode.
        Ok(method_bytes)
    }

    fn decompile<I>(mut bytecode_stream: &mut I) -> Result<ProgramMethod, MethodCompilerError>
    where
        I: Iterator<Item = u8>,
    {
        // Collect method name length
        let method_name_length = bytecode_stream
            .next()
            .ok_or(MethodCompilerError::NameLengthByteCollectError)?;

        if method_name_length > MAX_METHOD_NAME_LENGTH as u8
            || method_name_length < MIN_METHOD_NAME_LENGTH as u8
        {
            return Err(MethodCompilerError::InvalidNameLength);
        }

        // Collect method name
        let method_name_bytes: Vec<u8> = bytecode_stream
            .by_ref()
            .take(method_name_length as usize)
            .collect();

        // Convert method name bytes to string.
        let method_name = String::from_utf8_lossy(&method_name_bytes).to_string();

        // Collect method type bytecode.
        let method_type_bytecode = bytecode_stream
            .by_ref()
            .next()
            .ok_or(MethodCompilerError::MethodTypeByteCollectError)?;

        // Get method type from the bytecode.
        let method_type = MethodType::from_bytecode(method_type_bytecode)
            .ok_or(MethodCompilerError::InvalidMethodType)?;

        // Collect the number of call element types.
        let number_of_call_element_types = bytecode_stream
            .by_ref()
            .next()
            .ok_or(MethodCompilerError::CallElementTypesCountError)?;

        if number_of_call_element_types > MAX_METHOD_CALL_ELEMENT_TYPE_COUNT as u8
            || number_of_call_element_types < MIN_METHOD_CALL_ELEMENT_TYPE_COUNT as u8
        {
            return Err(MethodCompilerError::InvalidCallElementTypeLength);
        }

        // Collect call element types.
        let mut call_element_types = Vec::<CallElementType>::new();
        for _ in 0..number_of_call_element_types {
            let call_element_type = CallElementType::from_bytecode(&mut bytecode_stream)
                .ok_or(MethodCompilerError::InvalidCallElementType)?;

            call_element_types.push(call_element_type);
        }

        // Collect two bytes for opcodes count.
        let opcodes_count = u16::from_le_bytes([
            bytecode_stream.by_ref().next().unwrap(),
            bytecode_stream.by_ref().next().unwrap(),
        ]);

        if opcodes_count > MAX_METHOD_OPCODE_COUNT as u16
            || opcodes_count < MIN_METHOD_OPCODE_COUNT as u16
        {
            return Err(MethodCompilerError::OpcodeCountError);
        }

        // Collect opcodes.
        let mut opcodes = Vec::<Opcode>::new();
        for _ in 0..opcodes_count {
            let opcode = Opcode::decompile(&mut bytecode_stream)
                .map_err(|e| MethodCompilerError::OpcodeDecompileError(e))?;

            opcodes.push(opcode);
        }

        // Construct the method.
        let method = ProgramMethod::new(method_name, method_type, call_element_types, opcodes)
            .map_err(|e| MethodCompilerError::MethodConstructError(e))?;

        // Return the method.
        Ok(method)
    }
}
