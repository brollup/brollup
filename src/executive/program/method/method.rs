use super::{
    limits::{
        MAX_METHOD_CALL_ELEMENT_TYPE_COUNT, MAX_METHOD_NAME_LENGTH, MAX_METHOD_OPCODE_COUNT,
        MIN_METHOD_CALL_ELEMENT_TYPE_COUNT, MIN_METHOD_NAME_LENGTH, MIN_METHOD_OPCODE_COUNT,
    },
    method_error::{MethodConstructionError, ScriptValidationError},
    method_type::MethodType,
};
use crate::{
    constructive::calldata::element_type::CallElementType,
    executive::opcode::{
        op::{
            push::op_pushdata::OP_PUSHDATA,
            reserved::{op_reserved1::OP_RESERVED_1, op_reserved2::OP_RESERVED_2},
        },
        opcode::Opcode,
    },
};

/// A section of executable block in the `Contract`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramMethod {
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
        method_name: String,
        method_type: MethodType,
        call_element_types: Vec<CallElementType>,
        script: Vec<Opcode>,
    ) -> Result<Self, MethodConstructionError> {
        // Check method name byte length.
        if method_name.len() > MAX_METHOD_NAME_LENGTH || method_name.len() < MIN_METHOD_NAME_LENGTH
        {
            return Err(MethodConstructionError::MethodNameLengthError);
        }

        // Check call element type count.
        if call_element_types.len() > MAX_METHOD_CALL_ELEMENT_TYPE_COUNT
            || call_element_types.len() < MIN_METHOD_CALL_ELEMENT_TYPE_COUNT
        {
            return Err(MethodConstructionError::CallElementTypeCountError);
        }

        // Check opcode count.
        if script.len() > MAX_METHOD_OPCODE_COUNT || script.len() < MIN_METHOD_OPCODE_COUNT {
            return Err(MethodConstructionError::OpcodeCountError);
        }

        // Validate the script.
        if let Err(e) = Self::validate_script(&script) {
            return Err(MethodConstructionError::ScriptValidationError(e));
        }

        // Construct the method.
        let method = Self {
            method_name,
            method_type,
            call_element_types,
            script,
        };

        // Return the method.
        Ok(method)
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

    /// Validates the script.
    pub fn validate_script(script: &Vec<Opcode>) -> Result<(), ScriptValidationError> {
        for opcode in script {
            match opcode {
                // Check for reserved opcodes.
                Opcode::OP_RESERVED_1(OP_RESERVED_1) => {
                    return Err(ScriptValidationError::ReservedOpcodeEncounteredError);
                }
                Opcode::OP_RESERVED_2(OP_RESERVED_2) => {
                    return Err(ScriptValidationError::ReservedOpcodeEncounteredError);
                }
                // Check for non minimal push data.
                Opcode::OP_PUSHDATA(op_pushdata) => {
                    if op_pushdata.0.len() == 1 && !OP_PUSHDATA::check_minimal_push(&op_pushdata.0)
                    {
                        return Err(ScriptValidationError::NonMinimalDataPushError);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
