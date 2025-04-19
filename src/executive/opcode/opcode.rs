#![allow(non_camel_case_types)]

use super::op::{
    altstack::{op_fromaltstack::OP_FROMALTSTACK, op_toaltstack::OP_TOALTSTACK},
    bitwise::{
        op_and::OP_AND, op_equal::OP_EQUAL, op_equalverify::OP_EQUALVERIFY, op_invert::OP_INVERT,
        op_or::OP_OR, op_reverse::OP_REVERSE, op_xor::OP_XOR,
    },
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
    reserved::{
        op_reserved_1::OP_RESERVED_1, op_reserved_2::OP_RESERVED_2, op_reserved_3::OP_RESERVED_3,
        op_reserved_4::OP_RESERVED_4,
    },
    splice::{
        op_cat::OP_CAT, op_left::OP_LEFT, op_right::OP_RIGHT, op_size::OP_SIZE, op_split::OP_SPLIT,
    },
    stack::{
        op_2drop::OP_2DROP, op_2dup::OP_2DUP, op_2over::OP_2OVER, op_2rot::OP_2ROT,
        op_2swap::OP_2SWAP, op_3dup::OP_3DUP, op_depth::OP_DEPTH, op_drop::OP_DROP, op_dup::OP_DUP,
        op_ifdup::OP_IFDUP, op_nip::OP_NIP, op_over::OP_OVER, op_pick::OP_PICK, op_roll::OP_ROLL,
        op_rot::OP_ROT, op_swap::OP_SWAP, op_tuck::OP_TUCK,
    },
};
use std::fmt::{self, Display};

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
    OP_RESERVED_3(OP_RESERVED_3), //0x50
    OP_RESERVED_4(OP_RESERVED_4), //0x89
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
    OP_2DROP(OP_2DROP),
    OP_2DUP(OP_2DUP),
    OP_3DUP(OP_3DUP),
    OP_2OVER(OP_2OVER),
    OP_2ROT(OP_2ROT),
    OP_2SWAP(OP_2SWAP),
    OP_IFDUP(OP_IFDUP),
    OP_DEPTH(OP_DEPTH),
    OP_DROP(OP_DROP),
    OP_DUP(OP_DUP),
    OP_NIP(OP_NIP),
    OP_OVER(OP_OVER),
    OP_PICK(OP_PICK),
    OP_ROLL(OP_ROLL),
    OP_ROT(OP_ROT),
    OP_SWAP(OP_SWAP),
    OP_TUCK(OP_TUCK),
    // Splice
    OP_CAT(OP_CAT),
    OP_SPLIT(OP_SPLIT),
    OP_LEFT(OP_LEFT),
    OP_RIGHT(OP_RIGHT),
    OP_SIZE(OP_SIZE),
    // Bitwise
    OP_INVERT(OP_INVERT),
    OP_AND(OP_AND),
    OP_OR(OP_OR),
    OP_XOR(OP_XOR),
    OP_EQUAL(OP_EQUAL),
    OP_EQUALVERIFY(OP_EQUALVERIFY),
    OP_REVERSE(OP_REVERSE),
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
            Opcode::OP_RESERVED_3(_) => write!(f, "OP_RESERVED_3"),
            Opcode::OP_RESERVED_4(_) => write!(f, "OP_RESERVED_4"),
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
            Opcode::OP_2DROP(_) => write!(f, "OP_2DROP"),
            Opcode::OP_2DUP(_) => write!(f, "OP_2DUP"),
            Opcode::OP_3DUP(_) => write!(f, "OP_3DUP"),
            Opcode::OP_2OVER(_) => write!(f, "OP_2OVER"),
            Opcode::OP_2ROT(_) => write!(f, "OP_2ROT"),
            Opcode::OP_2SWAP(_) => write!(f, "OP_2SWAP"),
            Opcode::OP_IFDUP(_) => write!(f, "OP_IFDUP"),
            Opcode::OP_DEPTH(_) => write!(f, "OP_DEPTH"),
            Opcode::OP_DROP(_) => write!(f, "OP_DROP"),
            Opcode::OP_DUP(_) => write!(f, "OP_DUP"),
            Opcode::OP_NIP(_) => write!(f, "OP_NIP"),
            Opcode::OP_OVER(_) => write!(f, "OP_OVER"),
            Opcode::OP_PICK(_) => write!(f, "OP_PICK"),
            Opcode::OP_ROLL(_) => write!(f, "OP_ROLL"),
            Opcode::OP_ROT(_) => write!(f, "OP_ROT"),
            Opcode::OP_SWAP(_) => write!(f, "OP_SWAP"),
            Opcode::OP_TUCK(_) => write!(f, "OP_TUCK"),
            // Splice
            Opcode::OP_CAT(_) => write!(f, "OP_CAT"),
            Opcode::OP_SPLIT(_) => write!(f, "OP_SPLIT"),
            Opcode::OP_LEFT(_) => write!(f, "OP_LEFT"),
            Opcode::OP_RIGHT(_) => write!(f, "OP_RIGHT"),
            Opcode::OP_SIZE(_) => write!(f, "OP_SIZE"),
            // Bitwise
            Opcode::OP_INVERT(_) => write!(f, "OP_INVERT"),
            Opcode::OP_AND(_) => write!(f, "OP_AND"),
            Opcode::OP_OR(_) => write!(f, "OP_OR"),
            Opcode::OP_XOR(_) => write!(f, "OP_XOR"),
            Opcode::OP_EQUAL(_) => write!(f, "OP_EQUAL"),
            Opcode::OP_EQUALVERIFY(_) => write!(f, "OP_EQUALVERIFY"),
            Opcode::OP_REVERSE(_) => write!(f, "OP_REVERSE"),
        }
    }
}
