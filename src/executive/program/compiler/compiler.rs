use super::compiler_error::{ProgramCompileError, ProgramDecompileError};
use crate::executive::program::{
    method::{compiler::compiler::MethodCompiler, method::ProgramMethod},
    program::Program,
};

/// A trait for compiling and decompiling a program.
pub trait ProgramCompiler {
    /// Compiles the program into a bytecode.
    fn compile(&self) -> Result<Vec<u8>, ProgramCompileError>;
    /// Decompiles a program from a bytecode stream.
    fn decompile<I>(bytecode_stream: &mut I) -> Result<Program, ProgramDecompileError>
    where
        I: Iterator<Item = u8>;
}

impl ProgramCompiler for Program {
    fn compile(&self) -> Result<Vec<u8>, ProgramCompileError> {
        // Compile the script.
        let mut program_bytes = Vec::<u8>::new();

        // Encode program name byte length as u8.
        program_bytes.push(self.program_name().len() as u8);

        // Encode program name.
        program_bytes.extend(self.program_name().as_bytes());

        // Encode deployed by.
        program_bytes.extend(self.deployed_by());

        // Encode method count as u8.
        program_bytes.push(self.methods_len() as u8);

        // Encode methods.
        for method in self.methods().iter() {
            program_bytes.extend(
                method
                    .compile()
                    .map_err(|e| ProgramCompileError::MethodCompileError(e))?,
            );
        }

        // Return the bytecode.
        Ok(program_bytes)
    }

    fn decompile<I>(bytecode_stream: &mut I) -> Result<Program, ProgramDecompileError>
    where
        I: Iterator<Item = u8>,
    {
        // Decode program name byte length.
        let program_name_byte_length = bytecode_stream
            .next()
            .ok_or(ProgramDecompileError::NameLengthByteCollectError)?;

        // Collect program name bytes.
        let program_name_bytes: Vec<u8> = bytecode_stream
            .by_ref()
            .take(program_name_byte_length as usize)
            .collect();

        // Check if the program name bytes length is equal to the program name byte length.
        if program_name_bytes.len() != program_name_byte_length as usize {
            return Err(ProgramDecompileError::ProgramNameBytesCollectError);
        }

        // Collect 32 byte for the account key of the deployer.
        let deployed_by: [u8; 32] = bytecode_stream
            .by_ref()
            .take(32)
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| ProgramDecompileError::DeployedByBytesCollectError)?;

        // Convert program name bytes to string.
        let program_name = String::from_utf8_lossy(&program_name_bytes).to_string();

        // Collect method count byte.
        let method_count = bytecode_stream
            .next()
            .ok_or(ProgramDecompileError::MethodCountByteCollectError)?;

        // Collect methods.
        let mut methods = Vec::<ProgramMethod>::new();
        for _ in 0..method_count {
            let method = ProgramMethod::decompile(bytecode_stream)
                .map_err(|e| ProgramDecompileError::MethodDecompileError(e))?;
            methods.push(method);
        }

        // Construct the program.
        let program = Program::new(program_name, deployed_by, methods)
            .map_err(|e| ProgramDecompileError::ProgramConstructError(e))?;

        // Return the program.
        Ok(program)
    }
}
