use crate::executive::{
    opcode::compiler::compiler_error::{OpcodeCompileError, OpcodeDecompileError},
    program::method::method_error::MethodConstructionError,
};

/// The error that occurs when compiling a method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodCompileError {
    /// The opcode compile error.
    OpcodeCompileError(OpcodeCompileError),
}

/// The error that occurs when decompiling a method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodDecompileError {
    /// The method name length byte collect error.
    NameLengthByteCollectError,
    /// The method type byte collect error.
    MethodTypeByteCollectError,
    /// The method type is invalid.
    InvalidMethodType,
    /// The call element type count byte collect error.
    NumberOfCallElementTypesByteCollectError,
    /// The call element type is invalid.
    InvalidCallElementType,
    /// The opcode decompile error.
    OpcodeDecompileError(OpcodeDecompileError),
    /// The method construct error.
    MethodConstructError(MethodConstructionError),
}
