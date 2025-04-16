use std::fmt;

/// An error for encoding opcodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpcodeCompileError {
    /// The push data length is not valid.
    InvalidPushDataLength,
    /// The opcode is reserved.
    ReservedOpcodeError,
}

impl fmt::Display for OpcodeCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpcodeCompileError::InvalidPushDataLength => {
                write!(f, "Invalid push data length")
            }
            OpcodeCompileError::ReservedOpcodeError => {
                write!(f, "Reserved opcode encountered")
            }
        }
    }
}

/// An error for decoding opcodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpcodeDecompileError {
    /// The byte iterator error.
    ByteIteratorError,
    /// The push data length is not valid.
    InvalidPushDataLength,
    /// The reserved opcode error.
    ReservedOpcodeError,
    /// The non minimal data push error.
    NonMinimalDataPushError,
    /// The invalid data push tier error.
    InvalidDataPushTier,
}

impl fmt::Display for OpcodeDecompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpcodeDecompileError::ByteIteratorError => {
                write!(f, "Byte iterator error")
            }
            OpcodeDecompileError::InvalidPushDataLength => {
                write!(f, "Invalid push data length")
            }
            OpcodeDecompileError::ReservedOpcodeError => {
                write!(f, "Reserved opcode encountered")
            }
            OpcodeDecompileError::NonMinimalDataPushError => {
                write!(f, "Non-minimal data push encountered")
            }
            OpcodeDecompileError::InvalidDataPushTier => {
                write!(f, "Invalid data push tier")
            }
        }
    }
}
