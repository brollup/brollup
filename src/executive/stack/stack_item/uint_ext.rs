use super::item::StackItem;
use uint::construct_uint;

// Define a 256-bit unsigned integer type for the stack (4 x 64-bit words)
construct_uint! {
    pub struct StackUint(4);
}

/// Extension trait for converting between `StackItem` and `StackUint`.
///
/// Enables interpreting a `StackItem` as a `StackUint`, and constructing one from it.
///
/// Although a `StackItem` can represent a `StackUint`, it doesn't imply a fixed 32-byte size—stack items are variable-length,
/// and conversion preserves numeric value without applying padding or truncation.
pub trait StackItemUintExt {
    /// Converts a `StackItem` to a `StackUint`.
    fn to_uint(&self) -> Option<StackUint>;
    /// Converts a `StackUint` to a `StackItem`.
    fn from_uint(value: StackUint) -> StackItem;
}

impl StackItemUintExt for StackItem {
    fn to_uint(&self) -> Option<StackUint> {
        // Get the bytes of the stack item.
        let stack_item_bytes = self.bytes();

        // Get the `StackUint` value.
        let stack_uint = match stack_item_bytes.len() {
            0 => StackUint::zero(),
            len if len > 32 => return None,
            _ => StackUint::from_little_endian(stack_item_bytes),
        };

        // Return the `StackUint` value.
        Some(stack_uint)
    }

    fn from_uint(value: StackUint) -> StackItem {
        // If the value is zero, return an empty `StackItem`.
        if value == StackUint::zero() {
            return StackItem::new(vec![]);
        }

        // Create a buffer for the StackUint (256-bit unsigned integer).
        let mut buf = [0u8; 32];
        value.to_little_endian(&mut buf);

        // Get the minimal number of bytes required to represent the StackUint.
        let required_bytes = minimal_bytes_required(&value);

        // Return the `StackItem`.
        StackItem::new(buf[..required_bytes].to_vec())
    }
}

/// Returns the minimal number of bytes required to represent a `StackUint`.
fn minimal_bytes_required(value: &StackUint) -> usize {
    for byte_len in 1..=32 {
        let limit = StackUint::from(1u64) << (byte_len * 8);
        if *value < limit {
            return byte_len;
        }
    }
    32
}
