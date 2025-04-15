use super::compiler_error::{OpcodeCompileError, OpcodeDecompileError};
use crate::executive::opcode::op::flow::op_else::OP_ELSE;
use crate::executive::opcode::op::flow::op_endif::OP_ENDIF;
use crate::executive::opcode::op::flow::op_fail::OP_FAIL;
use crate::executive::opcode::op::flow::op_if::OP_IF;
use crate::executive::opcode::op::flow::op_nop::OP_NOP;
use crate::executive::opcode::op::flow::op_notif::OP_NOTIF;
use crate::executive::opcode::op::flow::op_returnall::OP_RETURNALL;
use crate::executive::opcode::op::flow::op_returnerr::OP_RETURNERR;
use crate::executive::opcode::op::flow::op_returnsome::OP_RETURNSOME;
use crate::executive::opcode::op::flow::op_verify::OP_VERIFY;
use crate::executive::opcode::op::push::op_10::OP_10;
use crate::executive::opcode::op::push::op_11::OP_11;
use crate::executive::opcode::op::push::op_12::OP_12;
use crate::executive::opcode::op::push::op_13::OP_13;
use crate::executive::opcode::op::push::op_14::OP_14;
use crate::executive::opcode::op::push::op_15::OP_15;
use crate::executive::opcode::op::push::op_16::OP_16;
use crate::executive::opcode::op::push::op_2::OP_2;
use crate::executive::opcode::op::push::op_3::OP_3;
use crate::executive::opcode::op::push::op_4::OP_4;
use crate::executive::opcode::op::push::op_5::OP_5;
use crate::executive::opcode::op::push::op_6::OP_6;
use crate::executive::opcode::op::push::op_7::OP_7;
use crate::executive::opcode::op::push::op_8::OP_8;
use crate::executive::opcode::op::push::op_9::OP_9;
use crate::executive::opcode::op::push::op_false::OP_FALSE;
use crate::executive::opcode::op::push::op_pushdata::OP_PUSHDATA;
use crate::executive::opcode::op::push::op_true::OP_TRUE;
use crate::executive::opcode::op::reserved::op_reserved1::OP_RESERVED_1;
use crate::executive::opcode::op::reserved::op_reserved2::OP_RESERVED_2;
use crate::executive::opcode::opcode::Opcode;

/// A trait for compiling and decompiling an opcode.
pub trait OpcodeCompiler {
    /// Compiles the opcode into a bytecode.
    fn compile(&self) -> Result<Vec<u8>, OpcodeCompileError>;
    /// Decompiles an opcode from a bytecode stream.
    fn decompile<I>(bytecode_stream: &mut I) -> Result<Opcode, OpcodeDecompileError>
    where
        I: Iterator<Item = u8>;
}

impl OpcodeCompiler for Opcode {
    fn compile(&self) -> Result<Vec<u8>, OpcodeCompileError> {
        match self {
            // Data push
            Opcode::OP_FALSE(OP_FALSE) => Ok(OP_FALSE::bytecode()),
            Opcode::OP_PUSHDATA(op_pushdata) => op_pushdata
                .compiled_bytes()
                .map(Ok)
                .unwrap_or_else(|| Err(OpcodeCompileError::InvalidPushDataLength)),
            Opcode::OP_RESERVED_1(OP_RESERVED_1) => Ok(OP_RESERVED_1::bytecode()),
            Opcode::OP_RESERVED_2(OP_RESERVED_2) => Ok(OP_RESERVED_2::bytecode()),
            Opcode::OP_TRUE(OP_TRUE) => Ok(OP_TRUE::bytecode()),
            Opcode::OP_2(OP_2) => Ok(OP_2::bytecode()),
            Opcode::OP_3(OP_3) => Ok(OP_3::bytecode()),
            Opcode::OP_4(OP_4) => Ok(OP_4::bytecode()),
            Opcode::OP_5(OP_5) => Ok(OP_5::bytecode()),
            Opcode::OP_6(OP_6) => Ok(OP_6::bytecode()),
            Opcode::OP_7(OP_7) => Ok(OP_7::bytecode()),
            Opcode::OP_8(OP_8) => Ok(OP_8::bytecode()),
            Opcode::OP_9(OP_9) => Ok(OP_9::bytecode()),
            Opcode::OP_10(OP_10) => Ok(OP_10::bytecode()),
            Opcode::OP_11(OP_11) => Ok(OP_11::bytecode()),
            Opcode::OP_12(OP_12) => Ok(OP_12::bytecode()),
            Opcode::OP_13(OP_13) => Ok(OP_13::bytecode()),
            Opcode::OP_14(OP_14) => Ok(OP_14::bytecode()),
            Opcode::OP_15(OP_15) => Ok(OP_15::bytecode()),
            Opcode::OP_16(OP_16) => Ok(OP_16::bytecode()),
            // Flow control
            Opcode::OP_NOP(OP_NOP) => Ok(OP_NOP::bytecode()),
            Opcode::OP_RETURNERR(OP_RETURNERR) => Ok(OP_RETURNERR::bytecode()),
            Opcode::OP_IF(OP_IF) => Ok(OP_IF::bytecode()),
            Opcode::OP_NOTIF(OP_NOTIF) => Ok(OP_NOTIF::bytecode()),
            Opcode::OP_RETURNALL(OP_RETURNALL) => Ok(OP_RETURNALL::bytecode()),
            Opcode::OP_RETURNSOME(OP_RETURNSOME) => Ok(OP_RETURNSOME::bytecode()),
            Opcode::OP_ELSE(OP_ELSE) => Ok(OP_ELSE::bytecode()),
            Opcode::OP_ENDIF(OP_ENDIF) => Ok(OP_ENDIF::bytecode()),
            Opcode::OP_VERIFY(OP_VERIFY) => Ok(OP_VERIFY::bytecode()),
            Opcode::OP_FAIL(OP_FAIL) => Ok(OP_FAIL::bytecode()),
            _ => Err(OpcodeCompileError::ReservedOpcodeError),
        }
    }

    fn decompile<I>(bytecode_stream: &mut I) -> Result<Opcode, OpcodeDecompileError>
    where
        I: Iterator<Item = u8>,
    {
        // Collect one byte from the bytecode stream.
        let byte = bytecode_stream
            .next()
            .ok_or(OpcodeDecompileError::ByteIteratorError)?;

        // Match the byte.
        match byte {
            // 0x00..0x60; Data pushes
            0x00 => Ok(Opcode::OP_FALSE(OP_FALSE)),
            0x01..=0x4b => {
                // Data length is the byte itself.
                let data_length = byte as usize;

                // Collect from bytestream data_length number of bytes.
                let data = bytecode_stream
                    .by_ref()
                    .take(data_length)
                    .collect::<Vec<u8>>();

                // Check if the data length is valid.
                if data.len() != data_length {
                    return Err(OpcodeDecompileError::InvalidPushDataLength);
                }

                // Check if the data is a minimal push.
                if data_length == 1 && !OP_PUSHDATA::check_minimal_push(&data) {
                    return Err(OpcodeDecompileError::NonMinimalDataPushError);
                }

                // Return the opcode.
                Ok(Opcode::OP_PUSHDATA(OP_PUSHDATA(data)))
            }
            0x4c => {
                // Data length is the next byte.
                let data_length = bytecode_stream
                    .next()
                    .ok_or(OpcodeDecompileError::ByteIteratorError)?
                    as usize;

                // Check if the data push is in tier 1.
                if data_length <= 75 {
                    // Should have been tier 0 (0x01..0x4b).
                    return Err(OpcodeDecompileError::InvalidDataPushTier);
                }

                // Collect from bytestream data_length number of bytes.
                let data = bytecode_stream
                    .by_ref()
                    .take(data_length)
                    .collect::<Vec<u8>>();

                // Check if the data length is valid.
                if data.len() != data_length {
                    return Err(OpcodeDecompileError::InvalidPushDataLength);
                }

                // Return the opcode.
                Ok(Opcode::OP_PUSHDATA(OP_PUSHDATA(data)))
            }
            0x4d => {
                // Collect two bytes for the data length in little endian order.
                let mut data_length = bytecode_stream
                    .next()
                    .ok_or(OpcodeDecompileError::ByteIteratorError)?
                    as u16;
                data_length |= (bytecode_stream
                    .next()
                    .ok_or(OpcodeDecompileError::ByteIteratorError)?
                    as u16)
                    << 8;

                // Check if the data push is in tier 2.
                if data_length <= 255 {
                    // Should have been tier 1 (0x4c).
                    return Err(OpcodeDecompileError::InvalidDataPushTier);
                }

                // Collect from bytestream data_length number of bytes.
                let data = bytecode_stream
                    .by_ref()
                    .take(data_length as usize)
                    .collect::<Vec<u8>>();

                // Check if the data length is valid.
                if data.len() != data_length as usize {
                    return Err(OpcodeDecompileError::InvalidPushDataLength);
                }

                // Return the opcode.
                Ok(Opcode::OP_PUSHDATA(OP_PUSHDATA(data)))
            }
            0x4e => Err(OpcodeDecompileError::ReservedOpcodeError),
            0x4f => Err(OpcodeDecompileError::ReservedOpcodeError),
            0x51 => Ok(Opcode::OP_TRUE(OP_TRUE)),
            0x52 => Ok(Opcode::OP_2(OP_2)),
            0x53 => Ok(Opcode::OP_3(OP_3)),
            0x54 => Ok(Opcode::OP_4(OP_4)),
            0x55 => Ok(Opcode::OP_5(OP_5)),
            0x56 => Ok(Opcode::OP_6(OP_6)),
            0x57 => Ok(Opcode::OP_7(OP_7)),
            0x58 => Ok(Opcode::OP_8(OP_8)),
            0x59 => Ok(Opcode::OP_9(OP_9)),
            0x5a => Ok(Opcode::OP_10(OP_10)),
            0x5b => Ok(Opcode::OP_11(OP_11)),
            0x5c => Ok(Opcode::OP_12(OP_12)),
            0x5d => Ok(Opcode::OP_13(OP_13)),
            0x5e => Ok(Opcode::OP_14(OP_14)),
            0x5f => Ok(Opcode::OP_15(OP_15)),
            0x60 => Ok(Opcode::OP_16(OP_16)),
            // 0x61..0x6a; Flow control
            0x61 => Ok(Opcode::OP_NOP(OP_NOP)),
            0x62 => Ok(Opcode::OP_RETURNERR(OP_RETURNERR)),
            0x63 => Ok(Opcode::OP_IF(OP_IF)),
            0x64 => Ok(Opcode::OP_NOTIF(OP_NOTIF)),
            0x65 => Ok(Opcode::OP_RETURNALL(OP_RETURNALL)),
            0x66 => Ok(Opcode::OP_RETURNSOME(OP_RETURNSOME)),
            0x67 => Ok(Opcode::OP_ELSE(OP_ELSE)),
            0x68 => Ok(Opcode::OP_ENDIF(OP_ENDIF)),
            0x69 => Ok(Opcode::OP_VERIFY(OP_VERIFY)),
            0x6a => Ok(Opcode::OP_FAIL(OP_FAIL)),
            _ => Err(OpcodeDecompileError::ReservedOpcodeError),
        }
    }
}
