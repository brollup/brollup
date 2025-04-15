use super::limits::{
    MAX_METHOD_COUNT, MAX_PROGRAM_NAME_LENGTH, MIN_METHOD_COUNT, MIN_PROGRAM_NAME_LENGTH,
};
use super::method::method::ProgramMethod;
use super::program_error::ProgramConstructionError;
use crate::constructive::valtype::atomic_val::AtomicVal;

/// A program associated with a `Contract`.
#[derive(Debug, Clone)]
pub struct Program {
    /// The program name.
    program_name: String,
    /// The methods to execute.
    methods: Vec<ProgramMethod>,
}

impl Program {
    /// Creates a new `Program` with the given program name and list of methods.
    pub fn new(
        program_name: String,
        methods: Vec<ProgramMethod>,
    ) -> Result<Self, ProgramConstructionError> {
        // Check program name length.
        if program_name.len() > MAX_PROGRAM_NAME_LENGTH
            || program_name.len() < MIN_PROGRAM_NAME_LENGTH
        {
            return Err(ProgramConstructionError::ProgramNameLengthError);
        }

        // Check method count.
        if methods.len() > MAX_METHOD_COUNT || methods.len() < MIN_METHOD_COUNT {
            return Err(ProgramConstructionError::MethodCountError);
        }

        // Construct the program.
        let program = Self {
            program_name,
            methods,
        };

        // Return the program.
        Ok(program)
    }

    /// Returns the program name.
    pub fn program_name(&self) -> &str {
        &self.program_name
    }

    /// Returns the method given the u8 index.
    /// Up to 256 methods are supported per program.
    pub fn method_by_index(&self, index: u8) -> Option<ProgramMethod> {
        self.methods.get(index as usize).cloned()
    }

    /// Returns the method by given `AtomicVal` index, rather than a u8.
    pub fn method_by_call_method(&self, call_method: AtomicVal) -> Option<ProgramMethod> {
        let method_index = call_method.value();
        self.method_by_index(method_index)
    }
}
