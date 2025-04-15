/// A section of executable block in the `Contract`.    
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptValidationError {
    /// Reserved opcode encountered error.
    ReservedOpcodeEncounteredError,
    /// Non minimal data push error.
    NonMinimalDataPushError,
}

/// A section of executable block in the `Contract`.    
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodConstructionError {
    /// Method name length error.
    MethodNameLengthError,
    /// Call element type count error.
    CallElementTypeCountError,
    /// Opcode count error.
    OpcodeCountError,
    /// Script validation error.
    ScriptValidationError(ScriptValidationError),
}
