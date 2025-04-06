#![allow(non_camel_case_types)]

use super::op::{
    flow::{
        op_else::OP_ELSE, op_endif::OP_ENDIF, op_fail::OP_FAIL, op_if::OP_IF, op_nop::OP_NOP,
        op_notif::OP_NOTIF, op_return::OP_RETURN, op_returnerr::OP_RETURNERR, op_verify::OP_VERIFY,
    },
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
    // Flow
    OP_NOP(OP_NOP),
    OP_IF(OP_IF),
    OP_NOTIF(OP_NOTIF),
    OP_ELSE(OP_ELSE),
    OP_ENDIF(OP_ENDIF),
    OP_VERIFY(OP_VERIFY),
    OP_RETURN(OP_RETURN),
    OP_FAIL(OP_FAIL),
    OP_RETURNERR(OP_RETURNERR),
    // Stack
    /// Pushes a copy of the topmost element onto the stack.
    OP_DUP(OP_DUP),
    /// Removes the topmost element from the stack.
    OP_DROP(OP_DROP),
    /// Concatenates the top two elements of the stack.
    OP_CAT(OP_CAT),
}

impl Opcode {
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
