use crate::executive::{
    opcode::compiler::compiler::OpcodeCompiler,
    program::method::{
        compiler::compiler_error::MethodCompilerError,
        limits::{MAX_METHOD_NAME_LENGTH, MAX_METHOD_SCRIPT_LENGTH},
        method::ProgramMethod,
    },
};

/// A trait for compiling and decompiling a program method.
pub trait MethodCompiler {
    /// Compiles the method into a bytecode.
    fn compile(&self) -> Result<Vec<u8>, MethodCompilerError>;
    /// Decompiles a method from a bytecode stream.
    fn decompile<I>(bytecode_stream: I) -> Result<ProgramMethod, MethodCompilerError>
    where
        I: Iterator<Item = u8>;
}

impl MethodCompiler for ProgramMethod {
    fn compile(&self) -> Result<Vec<u8>, MethodCompilerError> {
        // Compile the script.
        let mut method_bytes = Vec::<u8>::new();

        // Check method name length.
        if self.method_name().len() > MAX_METHOD_NAME_LENGTH {
            return Err(MethodCompilerError::NameLengthError);
        }

        // Check script length.
        if self.script().len() > MAX_METHOD_SCRIPT_LENGTH {
            return Err(MethodCompilerError::ScriptLengthError);
        }

        // Encode method name byte length as u8.
        method_bytes.push(self.method_name().len() as u8);

        // Encode method name.
        method_bytes.extend(self.method_name().as_bytes());

        // Encode method type.
        method_bytes.push(self.method_type().bytecode());

        // Check the number of call element types.
        if self.call_element_types().len() > u8::MAX as usize {
            return Err(MethodCompilerError::NumberOfCallElementTypesError);
        }

        // Encode the number of call element types as u8.
        method_bytes.push(self.call_element_types().len() as u8);

        // Encode individual call element types.
        for element_type in self.call_element_types().iter() {
            method_bytes.extend(element_type.bytecode());
        }

        // Check the number of opcodes.
        if self.script().len() > u16::MAX as usize {
            return Err(MethodCompilerError::NumberOfOpcodesError);
        }

        // Encode the number of opcodes as u16.
        method_bytes.extend((self.script().len() as u16).to_le_bytes());

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

    fn decompile<I>(_bytecode_stream: I) -> Result<ProgramMethod, MethodCompilerError>
    where
        I: Iterator<Item = u8>,
    {
        // TODO.
        Err(MethodCompilerError::NameLengthError)
    }
}
