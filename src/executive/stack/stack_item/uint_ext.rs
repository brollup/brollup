use super::item::StackItem;
use uint::construct_uint;

// A 256-bit unsigned integer representation for the `StackItem` (4 x 64-bit words).
construct_uint! {
    pub struct StackUint(4);
}

// A 320-bit unsigned integer type to deal with addmod and mulmod operations for `StackUint`.
construct_uint! {
    struct U320(5);
}

impl From<StackUint> for U320 {
    fn from(value: StackUint) -> Self {
        let mut result = U320::zero();
        for i in 0..4 {
            result.0[i] = value.0[i];
        }
        result
    }
}

impl From<U320> for StackUint {
    fn from(value: U320) -> Self {
        let mut result = StackUint::zero();
        for i in 0..4 {
            result.0[i] = value.0[i];
        }
        result
    }
}

impl StackUint {
    /// Add two `StackUint` values and return the result modulo MAX::U256.
    pub fn addmod(x: &StackUint, y: &StackUint) -> StackUint {
        let max = U320::from(StackUint::max_value());

        let x_as_u320 = U320::from(*x);
        let y_as_u320 = U320::from(*y);

        // Use overflowing_add to handle overflow safely
        let result = x_as_u320 + y_as_u320;

        let result_modulo_max = result % max;

        StackUint::from(result_modulo_max)
    }

    /// Multiply two `StackUint` values and return the result modulo MAX::U256.
    pub fn mulmod(x: &StackUint, y: &StackUint) -> StackUint {
        let max = U320::from(StackUint::max_value());
        let x_as_u320 = U320::from(*x);
        let y_as_u320 = U320::from(*y);

        let result = x_as_u320 * y_as_u320;
        let result_modulo_max = result % max;

        StackUint::from(result_modulo_max)
    }

    // function to convert to usize no panic, return none if overflow
    pub fn as_usize_safe(&self) -> Option<usize> {
        // Check length
        if self > &StackUint::from(usize::MAX) {
            return None;
        }

        // This cannot panic as we checked the length first.
        Some(self.as_usize())
    }
}

/// Trait for converting `StackUint` to `usize`, `u64`, and `u32` safely.
pub trait SafeConverter {
    fn u32(&self) -> Option<u32>;
    fn u64(&self) -> Option<u64>;
    fn usize(&self) -> Option<usize>;
}

impl SafeConverter for StackUint {
    fn u32(&self) -> Option<u32> {
        // Check length
        if self > &StackUint::from(u32::MAX) {
            return None;
        }

        // This cannot panic as we checked the length first.
        Some(self.as_u32())
    }

    fn u64(&self) -> Option<u64> {
        // Check length
        if self > &StackUint::from(u64::MAX) {
            return None;
        }

        // This cannot panic as we checked the length first.
        Some(self.as_u64())
    }

    fn usize(&self) -> Option<usize> {
        // Check length
        if self > &StackUint::from(usize::MAX) {
            return None;
        }

        // This cannot panic as we checked the length first.
        Some(self.as_usize())
    }
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
