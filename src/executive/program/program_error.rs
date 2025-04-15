/// A section of executable block in the `Contract`.    
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramConstructionError {
    /// Program name length error.
    ProgramNameLengthError,
    /// Method count error.
    MethodCountError,
}
