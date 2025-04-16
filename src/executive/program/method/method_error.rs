use std::fmt;

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

impl fmt::Display for MethodConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MethodConstructionError::MethodNameLengthError => {
                write!(f, "Method name length is invalid")
            }
            MethodConstructionError::CallElementTypeCountError => {
                write!(f, "Invalid call element type count")
            }
            MethodConstructionError::OpcodeCountError => {
                write!(f, "Invalid opcode count")
            }
            MethodConstructionError::ScriptValidationError(err) => {
                write!(f, "Script validation error: {}", err)
            }
        }
    }
}

/// A section of executable block in the `Contract`.    
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptValidationError {
    /// Reserved opcode encountered error.
    ReservedOpcodeEncounteredError,
    /// Non minimal data push error.
    NonMinimalDataPushError,
}

impl fmt::Display for ScriptValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptValidationError::ReservedOpcodeEncounteredError => {
                write!(f, "Reserved opcode encountered")
            }
            ScriptValidationError::NonMinimalDataPushError => {
                write!(f, "Non-minimal data push encountered")
            }
        }
    }
}
