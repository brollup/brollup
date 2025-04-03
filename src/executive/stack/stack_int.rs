use super::stack::StackItem;
use uint::construct_uint;

// Define a 256-bit integer type (4 x 64-bit words)
construct_uint! {
    pub struct U256(4);
}

pub trait StackInt {
    /// Convert the stack item to a `usize`.
    fn to_u256(&self) -> U256;
    /// Convert a `usize` to a stack item.
    fn from_u256(value: U256) -> Self;
}

impl StackInt for StackItem {
    fn to_u256(&self) -> U256 {
        let bytes = self.bytes();
        let len = bytes.len();

        if len > 32 {
            panic!("Invalid byte length for StackItem (must be 1-32 bytes)");
        }

        let mut arr = [0u8; 32]; // Zeroed buffer for U256
        arr[..len].copy_from_slice(&bytes); // Copy actual bytes

        U256::from_little_endian(&arr) // Convert to U256
    }

    fn from_u256(value: U256) -> Self {
        let mut bytes = vec![];
        let mut buf = [0u8; 32];
        value.to_little_endian(&mut buf); // Convert to bytes

        // Find the first non-zero byte from the end
        let mut required_bytes = 0;
        for i in (0..32).rev() {
            if buf[i] != 0 {
                required_bytes = i + 1;
                break;
            }
        }

        if required_bytes > 0 {
            bytes.extend_from_slice(&buf[..required_bytes]); // Store only needed bytes
        }
        StackItem::new(bytes)
    }
}
