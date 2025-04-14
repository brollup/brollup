use crate::executive::opcode::compiler::compiler_error::OpcodeCompileError;

/// The error that occurs when compiling a method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodCompilerError {
    /// The method name length is invalid.
    NameLengthError,
    /// The method script length is invalid.
    ScriptLengthError,
    /// The number of method call element types is invalid.
    NumberOfCallElementTypesError,
    /// The number of opcodes is invalid.
    NumberOfOpcodesError,
    /// The opcode compile error.
    OpcodeCompileError(OpcodeCompileError),
}
