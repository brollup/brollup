/// A section of executable block in the `Contract`.    
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramConstructionError {
    /// Program name length error.
    ProgramNameLengthError,
    /// Method count error.
    MethodCountError,
    /// Method validation error.
    MethodValidationError(MethodValidationError),
}

/// The error that occurs when validating the methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodValidationError {
    /// Duplicate method name error.
    DuplicateMethodNameError,
    /// All method types are internal error.
    AllMethodTypesAreInternal,
}
