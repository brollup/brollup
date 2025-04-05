#![allow(non_camel_case_types)]

use super::op::{
    push::{
        op_10::OP_10, op_11::OP_11, op_12::OP_12, op_13::OP_13, op_14::OP_14, op_15::OP_15,
        op_16::OP_16, op_2::OP_2, op_3::OP_3, op_4::OP_4, op_5::OP_5, op_6::OP_6, op_7::OP_7,
        op_8::OP_8, op_9::OP_9, op_false::OP_FALSE, op_pushdata::OP_PUSHDATA, op_true::OP_TRUE,
    },
    splice::op_cat::OP_CAT,
    stack::{op_drop::OP_DROP, op_dup::OP_DUP},
};

/// The set of opcodes that can be used in the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    // Push
    OP_FALSE(OP_FALSE),
    OP_TRUE(OP_TRUE),
    OP_2(OP_2),
    OP_3(OP_3),
    OP_4(OP_4),
    OP_5(OP_5),
    OP_6(OP_6),
    OP_7(OP_7),
    OP_8(OP_8),
    OP_9(OP_9),
    OP_10(OP_10),
    OP_11(OP_11),
    OP_12(OP_12),
    OP_13(OP_13),
    OP_14(OP_14),
    OP_15(OP_15),
    OP_16(OP_16),
    OP_PUSHDATA(OP_PUSHDATA),
    // Stack
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
            Opcode::OP_FALSE(_) => 0x00,
            Opcode::OP_TRUE(_) => 0x01,
            Opcode::OP_2(_) => 0x02,
            Opcode::OP_3(_) => 0x03,
            Opcode::OP_4(_) => 0x04,
            Opcode::OP_5(_) => 0x05,
            Opcode::OP_6(_) => 0x06,
            Opcode::OP_7(_) => 0x07,
            Opcode::OP_8(_) => 0x08,
            Opcode::OP_9(_) => 0x09,
            Opcode::OP_10(_) => 0x0a,
            Opcode::OP_11(_) => 0x0b,
            Opcode::OP_12(_) => 0x0c,
            Opcode::OP_13(_) => 0x0d,
            Opcode::OP_14(_) => 0x0e,
            Opcode::OP_15(_) => 0x0f,
            Opcode::OP_16(_) => 0x10,
            Opcode::OP_PUSHDATA(_) => 0x11,
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
