use crate::executive::{
    opcode::{codec::OpcodeEncoder, ops::OP_PUSHDATA_OPS},
    stack::{stack::StackHolder, stack_error::StackError, stack_item::item::StackItem},
};

/// The `OP_PUSHDATA1` opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_PUSHDATA(pub Vec<u8>);

impl OP_PUSHDATA {
    pub fn execute(self, stack_holder: &mut StackHolder) -> Result<(), StackError> {
        // The data to be pushed is the inner data in the OP_PUSHDATA struct.
        let item_to_push = StackItem::new(self.0);

        // Increment the ops counter.
        stack_holder.increment_ops(OP_PUSHDATA_OPS)?;

        // Push the item to the main stack.
        stack_holder.push(item_to_push)?;

        Ok(())
    }
}

impl OpcodeEncoder for OP_PUSHDATA {
    fn encode(&self) -> Vec<u8> {
        let mut encoded = vec![0x11];
        let data_length = self.0.len() as u16;
        encoded.extend(data_length.to_le_bytes());
        encoded.extend(self.0.clone());
        encoded
    }
}
