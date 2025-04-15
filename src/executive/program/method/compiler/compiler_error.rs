use crate::executive::{
    opcode::compiler::compiler_error::{OpcodeCompileError, OpcodeDecompileError},
    program::method::method_error::MethodConstructionError,
};

/// The error that occurs when compiling a method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodCompilerError {
    /// The method name length byte collect error.
    NameLengthByteCollectError,
    /// The method name length is invalid.
    InvalidNameLength,
    /// The method type byte collect error.
    MethodTypeByteCollectError,
    /// The method type is invalid.
    InvalidMethodType,
    /// The call element type length is invalid.
    InvalidCallElementTypeLength,
    /// The call element type is invalid.
    InvalidCallElementType,
    /// The number of method call element types is invalid.
    CallElementTypesCountError,
    /// The number of opcodes is invalid.
    OpcodeCountError,
    /// The opcode compile error.
    OpcodeCompileError(OpcodeCompileError),
    /// The opcode decompile error.
    OpcodeDecompileError(OpcodeDecompileError),
    /// The method construct error.
    MethodConstructError(MethodConstructionError),
}
