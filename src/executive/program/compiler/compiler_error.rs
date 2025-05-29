use crate::executive::program::{
    method::compiler::compiler_error::{MethodCompileError, MethodDecompileError},
    program_error::ProgramConstructionError,
};
use std::fmt;

/// The error that occurs when compiling a program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramCompileError {
    /// The method compile error.
    MethodCompileError(MethodCompileError),
}

impl fmt::Display for ProgramCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramCompileError::MethodCompileError(err) => {
                write!(f, "Method compile error: {}", err)
            }
        }
    }
}

/// The error that occurs when decompiling a program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramDecompileError {
    /// The program name length byte collect error.
    NameLengthByteCollectError,
    /// The program name bytes collect error.
    ProgramNameBytesCollectError,
    /// The deployed by bytes collect error.
    DeployedByBytesCollectError,
    /// The method count byte collect error.
    MethodCountByteCollectError,
    /// The method decompile error.
    MethodDecompileError(MethodDecompileError),
    /// The program construct error.
    ProgramConstructError(ProgramConstructionError),
}

impl fmt::Display for ProgramDecompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramDecompileError::NameLengthByteCollectError => {
                write!(f, "Failed to collect program name length byte")
            }
            ProgramDecompileError::ProgramNameBytesCollectError => {
                write!(f, "Failed to collect program name bytes")
            }
            ProgramDecompileError::DeployedByBytesCollectError => {
                write!(f, "Failed to collect deployed by bytes")
            }
            ProgramDecompileError::MethodCountByteCollectError => {
                write!(f, "Failed to collect method count byte")
            }
            ProgramDecompileError::MethodDecompileError(err) => {
                write!(f, "Method decompile error: {}", err)
            }
            ProgramDecompileError::ProgramConstructError(err) => {
                write!(f, "Program construction error: {}", err)
            }
        }
    }
}
