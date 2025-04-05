#![allow(non_camel_case_types)]

use super::op::{
    splice::op_cat::OP_CAT,
    stack::{op_drop::OP_DROP, op_dup::OP_DUP},
};

/// The set of opcodes that can be used in the stack.
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// Pushes a copy of the topmost element onto the stack.
    OP_DUP(OP_DUP),
    /// Removes the topmost element from the stack.
    OP_DROP(OP_DROP),
    /// Concatenates the top two elements of the stack.
    OP_CAT(OP_CAT),
}

impl Opcode {
    /// Returns the bytecode for the opcode.
    pub fn bytecode(&self) -> u8 {
        match self {
            Opcode::OP_DUP(_) => 0x76,
            Opcode::OP_DROP(_) => 0x75,
            Opcode::OP_CAT(_) => 0x7e,
        }
    }

    /// Returns the opcode for the given bytecode.
    pub fn from_bytecode(bytecode: u8) -> Option<Self> {
        match bytecode {
            0x76 => Some(Opcode::OP_DUP(OP_DUP)),
            0x75 => Some(Opcode::OP_DROP(OP_DROP)),
            0x7e => Some(Opcode::OP_CAT(OP_CAT)),
            _ => None,
        }
    }
}
