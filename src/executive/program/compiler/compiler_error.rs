use crate::executive::program::method::compiler::compiler_error::{
    MethodCompileError, MethodDecompileError,
};

/// The error that occurs when compiling a program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramCompileError {
    /// The method compile error.
    MethodCompileError(MethodCompileError),
}

/// The error that occurs when decompiling a program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramDecompileError {
    /// The method decompile error.
    MethodDecompileError(MethodDecompileError),
}
