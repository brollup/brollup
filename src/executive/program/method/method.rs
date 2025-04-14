use super::{
    limits::{MAX_METHOD_NAME_LENGTH, MAX_METHOD_SCRIPT_LENGTH},
    method_type::MethodType,
};
use crate::{
    constructive::calldata::element_type::CallElementType, executive::opcode::opcode::Opcode,
};

/// A section of executable block in the `Contract`.
#[derive(Debug, Clone)]
pub struct ProgramMethod {
    /// The contract id.
    contract_id: [u8; 32],
    /// The method name.
    method_name: String,
    /// The type of method.
    method_type: MethodType,
    /// Call element types.
    call_element_types: Vec<CallElementType>,
    /// The script to execute.
    script: Vec<Opcode>,
}

impl ProgramMethod {
    /// Create a new method.
    pub fn new(
        contract_id: [u8; 32],
        method_name: String,
        method_type: MethodType,
        call_element_types: Vec<CallElementType>,
        script: Vec<Opcode>,
    ) -> Option<Self> {
        // Check method name byte length.
        if method_name.len() > MAX_METHOD_NAME_LENGTH {
            return None;
        }

        // Check script byte length.
        if script.len() > MAX_METHOD_SCRIPT_LENGTH {
            return None;
        }

        // Construct the method.
        let method = Self {
            contract_id,
            method_name,
            method_type,
            call_element_types,
            script,
        };

        // Return the method.
        Some(method)
    }

    /// Returns the contract id.
    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    /// Returns the method name.
    pub fn method_name(&self) -> &str {
        &self.method_name
    }

    /// Returns the method type.
    pub fn method_type(&self) -> MethodType {
        self.method_type
    }

    /// Returns the call element types.
    pub fn call_element_types(&self) -> Vec<CallElementType> {
        self.call_element_types.clone()
    }

    /// Returns the script.
    pub fn script(&self) -> &Vec<Opcode> {
        &self.script
    }
}
