use crate::{calldata::element::CalldataElement, executive::stack::opcode::opcode::Opcode};

/// The type of method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodType {
    /// A callable method.
    Callable,
    /// An internal method.
    Internal,
    /// A read-only method.
    ReadOnly,
}

/// A section of executable block in the `Contract`.
#[derive(Debug, Clone)]
pub struct Method {
    /// The type of method.
    pub method_type: MethodType,
    /// The data elements to input to the method.
    pub calldata_elements: Vec<CalldataElement>,
    /// The program to execute.
    pub program: Vec<Opcode>,
}

impl Method {
    /// Create a new method.
    pub fn new(
        method_type: MethodType,
        calldata_elements: Vec<CalldataElement>,
        program: Vec<Opcode>,
    ) -> Self {
        Self {
            program,
            method_type,
            calldata_elements,
        }
    }

    /// Returns the method type.
    pub fn method_type(&self) -> MethodType {
        self.method_type
    }

    /// Returns the calldata elements.
    pub fn calldata_elements(&self) -> Vec<CalldataElement> {
        self.calldata_elements.clone()
    }

    /// Returns the program.
    pub fn program(&self) -> Vec<Opcode> {
        self.program.clone()
    }
}
