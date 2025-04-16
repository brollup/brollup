#![allow(non_camel_case_types)]

use std::fmt::{self, Display};

use super::op::{
    altstack::{op_fromaltstack::OP_FROMALTSTACK, op_toaltstack::OP_TOALTSTACK},
    flow::{
        op_else::OP_ELSE, op_endif::OP_ENDIF, op_fail::OP_FAIL, op_if::OP_IF, op_nop::OP_NOP,
        op_notif::OP_NOTIF, op_returnall::OP_RETURNALL, op_returnerr::OP_RETURNERR,
        op_returnsome::OP_RETURNSOME, op_verify::OP_VERIFY,
    },
    push::{
        op_10::OP_10, op_11::OP_11, op_12::OP_12, op_13::OP_13, op_14::OP_14, op_15::OP_15,
        op_16::OP_16, op_2::OP_2, op_3::OP_3, op_4::OP_4, op_5::OP_5, op_6::OP_6, op_7::OP_7,
        op_8::OP_8, op_9::OP_9, op_false::OP_FALSE, op_pushdata::OP_PUSHDATA, op_true::OP_TRUE,
    },
    reserved::{op_reserved1::OP_RESERVED_1, op_reserved2::OP_RESERVED_2},
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
    OP_RESERVED_1(OP_RESERVED_1), //0x4e
    OP_RESERVED_2(OP_RESERVED_2), //0x4f
    // Flow
    OP_NOP(OP_NOP),
    OP_RETURNERR(OP_RETURNERR),
    OP_IF(OP_IF),
    OP_NOTIF(OP_NOTIF),
    OP_ELSE(OP_ELSE),
    OP_ENDIF(OP_ENDIF),
    OP_VERIFY(OP_VERIFY),
    OP_RETURNALL(OP_RETURNALL),
    OP_RETURNSOME(OP_RETURNSOME),
    OP_FAIL(OP_FAIL),
    // Alts`tack
    OP_TOALTSTACK(OP_TOALTSTACK),
    OP_FROMALTSTACK(OP_FROMALTSTACK),
    // Stack
    OP_DUP(OP_DUP),
    OP_DROP(OP_DROP),
    // Splice
    OP_CAT(OP_CAT),
}

impl Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Data push
            Opcode::OP_FALSE(_) => write!(f, "OP_FALSE"),
            Opcode::OP_TRUE(_) => write!(f, "OP_TRUE"),
            Opcode::OP_2(_) => write!(f, "OP_2"),
            Opcode::OP_3(_) => write!(f, "OP_3"),
            Opcode::OP_4(_) => write!(f, "OP_4"),
            Opcode::OP_5(_) => write!(f, "OP_5"),
            Opcode::OP_6(_) => write!(f, "OP_6"),
            Opcode::OP_7(_) => write!(f, "OP_7"),
            Opcode::OP_8(_) => write!(f, "OP_8"),
            Opcode::OP_9(_) => write!(f, "OP_9"),
            Opcode::OP_10(_) => write!(f, "OP_10"),
            Opcode::OP_11(_) => write!(f, "OP_11"),
            Opcode::OP_12(_) => write!(f, "OP_12"),
            Opcode::OP_13(_) => write!(f, "OP_13"),
            Opcode::OP_14(_) => write!(f, "OP_14"),
            Opcode::OP_15(_) => write!(f, "OP_15"),
            Opcode::OP_16(_) => write!(f, "OP_16"),
            Opcode::OP_PUSHDATA(op_pushdata) => {
                write!(f, "OP_PUSHDATA 0x{}", hex::encode(&op_pushdata.0))
            }
            Opcode::OP_RESERVED_1(_) => write!(f, "OP_RESERVED_1"),
            Opcode::OP_RESERVED_2(_) => write!(f, "OP_RESERVED_2"),
            // Flow
            Opcode::OP_NOP(_) => write!(f, "OP_NOP"),
            Opcode::OP_RETURNERR(_) => write!(f, "OP_RETURNERR"),
            Opcode::OP_IF(_) => write!(f, "OP_IF"),
            Opcode::OP_NOTIF(_) => write!(f, "OP_NOTIF"),
            Opcode::OP_ELSE(_) => write!(f, "OP_ELSE"),
            Opcode::OP_ENDIF(_) => write!(f, "OP_ENDIF"),
            Opcode::OP_VERIFY(_) => write!(f, "OP_VERIFY"),
            Opcode::OP_RETURNALL(_) => write!(f, "OP_RETURNALL"),
            Opcode::OP_RETURNSOME(_) => write!(f, "OP_RETURNSOME"),
            Opcode::OP_FAIL(_) => write!(f, "OP_FAIL"),
            // Altstack
            Opcode::OP_TOALTSTACK(_) => write!(f, "OP_TOALTSTACK"),
            Opcode::OP_FROMALTSTACK(_) => write!(f, "OP_FROMALTSTACK"),
            // Stack
            Opcode::OP_DUP(_) => write!(f, "OP_DUP"),
            Opcode::OP_DROP(_) => write!(f, "OP_DROP"),
            Opcode::OP_CAT(_) => write!(f, "OP_CAT"),
        }
    }
}
