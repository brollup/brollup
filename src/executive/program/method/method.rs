use super::{
    limits::{
        MAX_METHOD_ARG_COUNT, MAX_METHOD_NAME_LENGTH, MAX_METHOD_OPCODE_COUNT,
        MIN_METHOD_ARG_COUNT, MIN_METHOD_NAME_LENGTH, MIN_METHOD_OPCODE_COUNT,
    },
    method_error::{MethodConstructionError, ScriptValidationError},
    method_type::MethodType,
};
use crate::{
    constructive::calldata::element_type::CallElementType,
    executive::opcode::{
        op::{
            push::op_pushdata::OP_PUSHDATA,
            reserved::{op_reserved_1::OP_RESERVED_1, op_reserved_2::OP_RESERVED_2},
        },
        opcode::Opcode,
    },
};
use serde_json::{Map, Value};

/// A section of executable block in the `Contract`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramMethod {
    /// The method name.
    method_name: String,
    /// The type of method.
    method_type: MethodType,
    /// Call element types.
    args: Vec<CallElementType>,
    /// The script to execute.
    script: Vec<Opcode>,
}

impl ProgramMethod {
    /// Create a new method.
    pub fn new(
        method_name: String,
        method_type: MethodType,
        args: Vec<CallElementType>,
        script: Vec<Opcode>,
    ) -> Result<Self, MethodConstructionError> {
        // Check method name byte length.
        if method_name.len() > MAX_METHOD_NAME_LENGTH || method_name.len() < MIN_METHOD_NAME_LENGTH
        {
            return Err(MethodConstructionError::MethodNameLengthError);
        }

        // Check arg count.
        if args.len() > MAX_METHOD_ARG_COUNT || args.len() < MIN_METHOD_ARG_COUNT {
            return Err(MethodConstructionError::ArgCountError);
        }

        // Check opcode count.
        if script.len() > MAX_METHOD_OPCODE_COUNT || script.len() < MIN_METHOD_OPCODE_COUNT {
            return Err(MethodConstructionError::OpcodeCountError);
        }

        // Construct the method.
        let method = Self {
            method_name,
            method_type,
            args,
            script,
        };

        // Validate the script.
        if let Err(e) = method.validate_script() {
            return Err(MethodConstructionError::ScriptValidationError(e));
        }

        // Validate the args.
        if !method.validate_args() {
            return Err(MethodConstructionError::ArgValidationError);
        }

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
    pub fn args(&self) -> Vec<CallElementType> {
        self.args.clone()
    }

    /// Returns the script.
    pub fn script(&self) -> &Vec<Opcode> {
        &self.script
    }

    /// Validates the script.
    pub fn validate_script(&self) -> Result<(), ScriptValidationError> {
        for opcode in self.script.iter() {
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

    /// Validates the args.
    pub fn validate_args(&self) -> bool {
        // More than ONE payable is not allowed.
        self.args
            .iter()
            .filter(|arg| **arg == CallElementType::Payable)
            .count()
            <= 1
    }

    /// Returns the method as a JSON object.
    pub fn json(&self) -> Value {
        // Convert the call element types to JSON.
        let args: Vec<Value> = self
            .args
            .iter()
            .map(|element_type| Value::String(element_type.to_string()))
            .collect();

        // Construct the method JSON object.
        let mut obj = Map::new();
        obj.insert(
            "method_name".to_string(),
            Value::String(self.method_name.clone()),
        );

        // Add the method type to the method JSON object.
        obj.insert(
            "method_type".to_string(),
            Value::String(self.method_type.to_string()),
        );

        // Add the call element types to the method JSON object.
        obj.insert("args".to_string(), Value::Array(args));

        // Convert the script to JSON.
        let script: Vec<Value> = self
            .script
            .iter()
            .map(|opcode| Value::String(opcode.to_string()))
            .collect();

        // Add the script to the method JSON object.
        obj.insert("script".to_string(), Value::Array(script));

        // Return the method JSON object.
        Value::Object(obj)
    }
}
