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
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError>;
}

/// An error for encoding opcodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpcodeEncoderError {
    /// The push data length is not valid.
    InvalidPushDataLength,
}

/// An error for decoding opcodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpcodeDecoderError {
    /// The byte iterator error.
    ByteIteratorError,
    /// The push data length is not valid.
    InvalidPushDataLength,
    /// The reserved opcode error.
    ReservedOpcodeError,
    /// The non minimal data push error.
    NonMinimalDataPushError,
    /// The invalid data push tier error.
    InvalidDataPushTier,
}

/// A trait for decoding opcodes from an iterator.
pub trait OpcodeDecoder<I>
where
    I: Iterator<Item = u8>,
{
    fn decode(&mut self) -> Result<Opcode, OpcodeDecoderError>;
}

impl<I> OpcodeDecoder<I> for I
where
    I: Iterator<Item = u8>,
{
    fn decode(&mut self) -> Result<Opcode, OpcodeDecoderError> {
        let byte = self.next().ok_or(OpcodeDecoderError::ByteIteratorError)?; // get the next byte from iterator

        match byte {
            0x00 => Ok(Opcode::OP_FALSE(OP_FALSE)),
            0x01..=0x4b => {
                // Data length is the byte itself.
                let data_length = byte as usize;

                // Collect the data.
                let data = self.take(data_length).collect::<Vec<u8>>();

                // Check if the data length is valid.
                if data.len() != data_length {
                    return Err(OpcodeDecoderError::InvalidPushDataLength);
                }

                // Check if the data is a minimal push.
                if data_length == 1 && !check_minimal_push(&data) {
                    return Err(OpcodeDecoderError::NonMinimalDataPushError);
                }

                // Return the opcode.
                Ok(Opcode::OP_PUSHDATA(OP_PUSHDATA(data)))
            }
            0x4c => {
                // Data length is the next byte.
                let data_length =
                    self.next().ok_or(OpcodeDecoderError::ByteIteratorError)? as usize;

                // Check if the data push is in tier 1.
                if data_length <= 75 {
                    // Should have been tier 0 (0x01..0x4b).
                    return Err(OpcodeDecoderError::InvalidDataPushTier);
                }

                // Collect the data.
                let data = self.take(data_length).collect::<Vec<u8>>();

                // Check if the data length is valid.
                if data.len() != data_length {
                    return Err(OpcodeDecoderError::InvalidPushDataLength);
                }

                // Return the opcode.
                Ok(Opcode::OP_PUSHDATA(OP_PUSHDATA(data)))
            }
            0x4d => {
                // Collect two bytes for the data length in little endian order.
                let mut data_length =
                    self.next().ok_or(OpcodeDecoderError::ByteIteratorError)? as u16;
                data_length |=
                    (self.next().ok_or(OpcodeDecoderError::ByteIteratorError)? as u16) << 8;

                // Check if the data push is in tier 2.
                if data_length <= 255 {
                    // Should have been tier 1 (0x4c).
                    return Err(OpcodeDecoderError::InvalidDataPushTier);
                }

                // Collect the data.
                let data = self.take(data_length as usize).collect::<Vec<u8>>();

                // Check if the data length is valid.
                if data.len() != data_length as usize {
                    return Err(OpcodeDecoderError::InvalidPushDataLength);
                }

                // Return the opcode.
                Ok(Opcode::OP_PUSHDATA(OP_PUSHDATA(data)))
            }
            0x4e => Err(OpcodeDecoderError::ReservedOpcodeError),
            0x4f => Err(OpcodeDecoderError::ReservedOpcodeError),
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
            _ => Err(OpcodeDecoderError::ReservedOpcodeError),
        }
    }
}

/// Check if the push data is a minimal push.
fn check_minimal_push(data: &[u8]) -> bool {
    match data.len() {
        // Data should have been encoded as OP_FALSE (OP_0).
        0 => false,
        // Data might or might not be a minimal push.
        1 => {
            // Check if the data is a minimal push.
            match data.get(0) {
                // Should have been encoded as OP_0..OP_16.
                Some(value) if value >= &0 && value <= &16 => false,
                // Validation passes otherwise.
                _ => true,
            }
        }
        // Minimal push values are always single byte values.
        // Check for multi-byte values are not needed.
        _ => true,
    }
}
