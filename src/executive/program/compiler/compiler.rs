use super::compiler_error::{ProgramCompileError, ProgramDecompileError};
use crate::executive::program::program::Program;

/// A trait for compiling and decompiling a program.
pub trait ProgramCompiler {
    /// Compiles the program into a bytecode.
    fn compile(&self) -> Result<Vec<u8>, ProgramCompileError>;
    /// Decompiles a program from a bytecode stream.
    fn decompile<I>(bytecode_stream: &mut I) -> Result<Program, ProgramDecompileError>
    where
        I: Iterator<Item = u8>;
}
