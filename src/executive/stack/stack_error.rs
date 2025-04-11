#[derive(Debug, Clone, Copy)]
pub enum StackError {
    /// The stack is empty.
    EmptyStack,
    /// The stack item is too large.
    StackItemTooLarge,
    /// The stack is too large.
    StackTooLarge,
    /// The pick index is out of bounds.
    PickIndexError(u32),
    /// The remove index is out of bounds.
    RemoveIndexError(u32),
    // Equalverify error.
    MandatoryEqualVerifyError,
    // Verify error.
    MandatoryVerifyError,
    // Invalid memory key length.
    InvalidMemoryKeyLength(u8),
    // Invalid memory value length.
    InvalidMemoryValueLength(u8),
    // Invalid storage key length.
    InvalidStorageKeyLength(u8),
    // Invalid storage value length.
    InvalidStorageValueLength(u8),
    // Memory size limit exceeded.
    ContractMemorySizeLimitExceeded,
    // Internal ops budget exceeded.
    InternalOpsBudgetExceeded,
    // External ops limit exceeded.
    ExternalOpsLimitExceeded,
    // StackUint max overflow error.
    StackUintMaxOverflowError,
    // Division by zero error.
    DivisionByZeroError,
    // Fail error.
    FailError,
    // Error string conversion error.
    ErrorStringConversionError,
    // OP_ELSE encountered without preceding OP_IF/OP_NOTIF error.
    OPElseEncounteredWithoutPrecedingIfNotif,
    // OP_ELSE encountered with preceding OP_ELSE error.
    OPElseEncounteredWithPrecedingAnotherOPElse,
    // OP_ELSE encountered without preceding execution flag error.
    OPElseEncounteredWithoutPrecedingExecutionFlag,
}
