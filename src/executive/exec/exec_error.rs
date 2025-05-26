use crate::executive::stack::{stack_error::StackError, stack_item::StackItem};
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
    /// Invalid external call attempt as internal call error.
    ExternalCallAttemptAsInternalError,
    /// Return error item error.
    ReturnErrorFromStackError(StackItem),
    /// Reserved opcode encountered error.
    ReservedOpcodeEncounteredError,
    /// Arg type mismatch error.
    ArgTypeMismatchError,
    /// Min payable allocation error.
    MinPayableAllocationError,
    /// Payable allocation caller is not an account error.
    PayableAllocationCallerIsNotAnAccountError,
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
            ExecutionError::ExternalCallAttemptAsInternalError => {
                write!(f, "External call attempt as internal call")
            }
            ExecutionError::ReturnErrorFromStackError(error) => {
                write!(f, "Return error from stack: {:?}", error)
            }
            ExecutionError::ReservedOpcodeEncounteredError => {
                write!(f, "Reserved opcode encountered")
            }
            ExecutionError::ArgTypeMismatchError => {
                write!(f, "Arg type mismatch")
            }
            ExecutionError::MinPayableAllocationError => {
                write!(f, "Min payable allocation error")
            }
            ExecutionError::PayableAllocationCallerIsNotAnAccountError => {
                write!(f, "Payable allocation caller is not an account")
            }
        }
    }
}
