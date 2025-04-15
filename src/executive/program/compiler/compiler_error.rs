use crate::executive::program::{
    method::compiler::compiler_error::{MethodCompileError, MethodDecompileError},
    program_error::ProgramConstructionError,
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
    /// The program name length byte collect error.
    NameLengthByteCollectError,
    /// The program name bytes collect error.
    ProgramNameBytesCollectError,
    /// The method count byte collect error.
    MethodCountByteCollectError,
    /// The method decompile error.
    MethodDecompileError(MethodDecompileError),
    /// The program construct error.
    ProgramConstructError(ProgramConstructionError),
}
