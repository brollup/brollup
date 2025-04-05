use super::{
    op::push::{
        op_10::OP_10, op_11::OP_11, op_12::OP_12, op_13::OP_13, op_14::OP_14, op_15::OP_15,
        op_16::OP_16, op_2::OP_2, op_3::OP_3, op_4::OP_4, op_5::OP_5, op_6::OP_6, op_7::OP_7,
        op_8::OP_8, op_9::OP_9, op_false::OP_FALSE, op_pushdata::OP_PUSHDATA, op_true::OP_TRUE,
    },
    opcode::Opcode,
};
/// A trait for encoding opcodes.
pub trait OpcodeEncoder {
    /// Encode the opcode into a vector of bytes.
    fn encode(&self) -> Vec<u8>;
}

/// A trait for decoding opcodes from an iterator.
pub trait OpcodeDecoder<I>
where
    I: Iterator<Item = u8>,
{
    fn decode(&mut self) -> Option<Opcode>;
}

impl<I> OpcodeDecoder<I> for I
where
    I: Iterator<Item = u8>,
{
    fn decode(&mut self) -> Option<Opcode> {
        let byte = self.next()?; // get the next byte from iterator

        match byte {
            0x00 => Some(Opcode::OP_FALSE(OP_FALSE)),
            0x01 => Some(Opcode::OP_TRUE(OP_TRUE)),
            0x02 => Some(Opcode::OP_2(OP_2)),
            0x03 => Some(Opcode::OP_3(OP_3)),
            0x04 => Some(Opcode::OP_4(OP_4)),
            0x05 => Some(Opcode::OP_5(OP_5)),
            0x06 => Some(Opcode::OP_6(OP_6)),
            0x07 => Some(Opcode::OP_7(OP_7)),
            0x08 => Some(Opcode::OP_8(OP_8)),
            0x09 => Some(Opcode::OP_9(OP_9)),
            0x0a => Some(Opcode::OP_10(OP_10)),
            0x0b => Some(Opcode::OP_11(OP_11)),
            0x0c => Some(Opcode::OP_12(OP_12)),
            0x0d => Some(Opcode::OP_13(OP_13)),
            0x0e => Some(Opcode::OP_14(OP_14)),
            0x0f => Some(Opcode::OP_15(OP_15)),
            0x10 => Some(Opcode::OP_16(OP_16)),
            0x11 => {
                // Collect two bytes for the data length in little endian order.
                let mut data_length = self.next()? as u16;
                data_length |= (self.next()? as u16) << 8;

                // Collect the data.
                let data = self.take(data_length as usize).collect::<Vec<u8>>();

                // Return the opcode.
                Some(Opcode::OP_PUSHDATA(OP_PUSHDATA(data)))
            }
            _ => None,
        }
    }
}
