/// An error for encoding opcodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpcodeCompileError {
    /// The push data length is not valid.
    InvalidPushDataLength,
    /// The opcode is reserved.
    ReservedOpcodeError,
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
