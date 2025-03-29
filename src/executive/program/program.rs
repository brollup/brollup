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

    /// Returns the method given the u8 index.
    /// Up to 256 methods are supported per program.
    pub fn method_by_index(&self, index: u8) -> Option<Method> {
        self.methods.get(index as usize).cloned()
    }

    /// Returns the method by given `AtomicVal` index, rather than a u8.
    ///
    /// `AtomicVal` is a compact value representing the method's index.
    /// A CPE-decoded `AtomicVal` support 8 values, meaning first 8 methods should be organized as callable.
    /// This means the first 8 methods in a deployed `Contract` are the only ones that can be called by an `Account`.
    ///
    /// A `Contract` calling another `Contract` is not bound by this constraint; it can call any of its (up to 256) methods.
    pub fn method_by_call_method(&self, call_method: AtomicVal) -> Option<Method> {
        let call_method_index = call_method.value();
        self.methods.get(call_method_index as usize).cloned()
    }
}
