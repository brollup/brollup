use super::method::method::{Method, MethodType};
use crate::valtype::atomic_val::AtomicVal;

/// A program associated with a `Contract`.
#[derive(Debug, Clone)]
pub struct Program {
    /// The contract id.
    contract_id: [u8; 32],
    /// The methods to execute.
    methods: Vec<Method>,
}

impl Program {
    /// Creates a new `Program` with the given contract id and list of methods.
    pub fn new(contract_id: [u8; 32], methods: Vec<Method>) -> Self {
        Self {
            contract_id,
            methods,
        }
    }

    /// Returns the contract id.
    pub fn contract_id(&self) -> [u8; 32] {
        self.contract_id
    }

    /// Returns the methods.
    pub fn methods(&self) -> Vec<Method> {
        self.methods.clone()
    }

    /// Returns the callable methods.
    pub fn callable_methods(&self) -> Vec<Method> {
        self.methods
            .iter()
            .filter(|method| method.method_type == MethodType::Callable)
            .cloned()
            .collect()
    }

    /// Returns the callable method by index.
    /// AtomicVal support from 8 values, meaning 8 callable methods are supported per program.
    pub fn callable_method_by_call_method(&self, call_method: AtomicVal) -> Option<Method> {
        let call_method_index = call_method.value();

        self.methods.get(call_method_index as usize).cloned()
    }

    /// Returns the read-only methods.
    pub fn read_only_methods(&self) -> Vec<Method> {
        self.methods
            .iter()
            .filter(|method| method.method_type == MethodType::ReadOnly)
            .cloned()
            .collect()
    }

    /// Returns the read-only method by index.
    /// Up to 256 read-only methods are supported per program.
    pub fn read_only_method_by_index(&self, index: u8) -> Option<Method> {
        self.read_only_methods().get(index as usize).cloned()
    }
}
