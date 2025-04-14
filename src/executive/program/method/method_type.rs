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

impl MethodType {
    /// Returns the bytecode of the method type.
    pub fn bytecode(&self) -> u8 {
        match self {
            MethodType::Callable => 0x00,
            MethodType::Internal => 0x01,
            MethodType::ReadOnly => 0x02,
        }
    }

    /// Returns the method type from the bytecode.
    pub fn from_bytecode(bytecode: u8) -> Option<Self> {
        match bytecode {
            0x00 => Some(MethodType::Callable),
            0x01 => Some(MethodType::Internal),
            0x02 => Some(MethodType::ReadOnly),
            _ => None,
        }
    }
}
