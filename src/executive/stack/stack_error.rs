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
    // Stack uitn conversion error.
    StackUintConversionError,
    // Fail error.
    FailError,
    // OP_ELSE encountered with preceding OP_ELSE error.
    OPElseEncounteredWithPrecedingAnotherOPElse,
    // OP_ELSE encountered without preceding flow encounter error.
    OPElseEncounteredWithoutPrecedingFlowEncounter,
    // Reserved opcode encountered error.
    ReservedOpcodeEncounteredError,
    // Split index error.
    SplitIndexError,
    // BLAKE2b var output size error.
    BLAKE2bVarOutputSizeError,
    // BLAKE2s var output size error.
    BLAKE2sVarOutputSizeError,
    // Invalid secp scalar bytes.
    InvalidSecpScalarBytes,
    // Invalid secp point bytes.
    InvalidSecpPointBytes,
    // Invalid schnorr public key bytes.
    InvalidSchnorrPublicKeyBytes,
    // Invalid schnorr message bytes.
    InvalidSchnorrMessageBytes,
    // Invalid schnorr signature bytes.
    InvalidSchnorrSignatureBytes,
    // Invalid BLS public key bytes.
    InvalidBLSPublicKeyBytes,
    // Invalid BLS message bytes.
    InvalidBLSMessageBytes,
    // Invalid BLS signature bytes.
    InvalidBLSSignatureBytes,
}
