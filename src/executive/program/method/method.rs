use crate::executive::opcode::opcode::Opcode;

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
pub struct ProgramMethod {
    /// The contract id.
    contract_id: [u8; 32],
    /// The type of method.
    method_type: MethodType,
    /// The script to execute.
    script: Vec<Opcode>,
}

impl ProgramMethod {
    /// Create a new method.
    pub fn new(contract_id: [u8; 32], method_type: MethodType, script: Vec<Opcode>) -> Self {
        Self {
            contract_id,
            method_type,
            script,
        }
    }

    /// Returns the contract id.
    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    /// Returns the method type.
    pub fn method_type(&self) -> MethodType {
        self.method_type
    }

    /// Returns the script.
    pub fn script(&self) -> Vec<Opcode> {
        self.script.clone()
    }
}
