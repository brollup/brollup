use crate::executive::stack::stack_error::StackError;
use std::fmt;

/// A section of executable block in the `Contract`.    
#[derive(Debug, Clone)]
pub enum ExecutionError {
    /// Method not found at index error.
    MethodNotFoundAtIndexError(u8),
    /// Stack holder initialization error.
    StackHolderInitializationError(StackError),
    /// Opcode execution error.
    OpcodeExecutionError(StackError),
    /// Method not returned any items error.
    MethodNotReturnedAnyItemsError,
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::MethodNotFoundAtIndexError(index) => {
                write!(f, "Method not found at index: {}", index)
            }
            ExecutionError::StackHolderInitializationError(error) => {
                write!(f, "Stack holder initialization error: {:?}", error)
            }
            ExecutionError::OpcodeExecutionError(error) => {
                write!(f, "Opcode execution error: {:?}", error)
            }
            ExecutionError::MethodNotReturnedAnyItemsError => {
                write!(f, "Method not returned any items")
            }
        }
    }
}
