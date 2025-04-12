use super::op_10::OP_10;
use super::op_11::OP_11;
use super::op_12::OP_12;
use super::op_13::OP_13;
use super::op_14::OP_14;
use super::op_15::OP_15;
use super::op_16::OP_16;
use super::op_2::OP_2;
use super::op_3::OP_3;
use super::op_4::OP_4;
use super::op_5::OP_5;
use super::op_6::OP_6;
use super::op_7::OP_7;
use super::op_8::OP_8;
use super::op_9::OP_9;
use super::op_false::OP_FALSE;
use super::op_true::OP_TRUE;
use crate::executive::{
    opcode::{
        codec::{OpcodeEncoder, OpcodeEncoderError},
        ops::OP_PUSHDATA_OPS,
    },
    stack::{
        limits::MAX_STACK_ITEM_SIZE, stack_error::StackError, stack_holder::StackHolder,
        stack_item::StackItem,
    },
};

/// Pushes data to the main stack.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_PUSHDATA(pub Vec<u8>);

impl OP_PUSHDATA {
    pub fn execute(&self, stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // The data to be pushed is the inner data in the OP_PUSHDATA struct.
        let item_to_push = StackItem::new(self.0.to_owned());

        // Check if the data length is valid.
        if item_to_push.len() > MAX_STACK_ITEM_SIZE {
            return Err(StackError::StackItemTooLarge);
        }

        // Increment the ops counter.
        stack_holder.increment_ops(OP_PUSHDATA_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}

/// Implement the `OpcodeEncoder` trait for `OP_PUSHDATA`.
impl OpcodeEncoder for OP_PUSHDATA {
    fn encode(&self) -> Result<Vec<u8>, OpcodeEncoderError> {
        // Determine the pushdata type.
        match self.0.len() {
            0 => Ok(OP_FALSE.encode()?),
            1 => {
                // Get the value.
                let value: u8 = self.0[0];

                // Check value for minimal encoding.
                match value {
                    0 => Ok(OP_FALSE.encode()?),
                    1 => Ok(OP_TRUE.encode()?),
                    2 => Ok(OP_2.encode()?),
                    3 => Ok(OP_3.encode()?),
                    4 => Ok(OP_4.encode()?),
                    5 => Ok(OP_5.encode()?),
                    6 => Ok(OP_6.encode()?),
                    7 => Ok(OP_7.encode()?),
                    8 => Ok(OP_8.encode()?),
                    9 => Ok(OP_9.encode()?),
                    10 => Ok(OP_10.encode()?),
                    11 => Ok(OP_11.encode()?),
                    12 => Ok(OP_12.encode()?),
                    13 => Ok(OP_13.encode()?),
                    14 => Ok(OP_14.encode()?),
                    15 => Ok(OP_15.encode()?),
                    16 => Ok(OP_16.encode()?),
                    _ => {
                        // Initialize the encoded vector.
                        let mut encoded = Vec::<u8>::with_capacity(self.0.len() + 1);

                        // Push the data length.
                        encoded.push(self.0.len() as u8);

                        // Push the data.
                        encoded.extend(self.0.clone());

                        // Return the encoded vector.
                        Ok(encoded)
                    }
                }
            }
            2..=75 => {
                // Initialize the encoded vector.
                let mut encoded = Vec::<u8>::with_capacity(self.0.len() + 1);

                // Push the data length.
                encoded.push(self.0.len() as u8);

                // Push the data.
                encoded.extend(self.0.clone());

                // Return the encoded vector.
                Ok(encoded)
            }
            76..=255 => {
                // Initialize the encoded vector.
                let mut encoded = Vec::<u8>::with_capacity(self.0.len() + 2);

                // Push 0x4c to the encoded vector.
                encoded.push(0x4c);

                // Push the data length.
                encoded.push(self.0.len() as u8);

                // Push the data.
                encoded.extend(self.0.clone());

                // Return the encoded vector.
                Ok(encoded)
            }
            256..=65535 => {
                // Initialize the encoded vector.
                let mut encoded = Vec::<u8>::with_capacity(self.0.len() + 3);

                // Push 0x4d to the encoded vector.
                encoded.push(0x4d);

                // Push the data length as a little endian u16.
                let data_length = self.0.len() as u16;
                encoded.push((data_length & 0xff) as u8);
                encoded.push((data_length >> 8) as u8);

                // Push the data.
                encoded.extend(self.0.clone());

                // Return the encoded vector.
                Ok(encoded)
            }
            _ => Err(OpcodeEncoderError::InvalidPushDataLength),
        }
    }
}
