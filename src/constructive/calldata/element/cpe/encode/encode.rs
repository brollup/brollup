use crate::constructive::calldata::element::element::CallElement;
use bit_vec::BitVec;

impl CallElement {
    /// Encodes the `CallElement` as a bit vector.
    pub fn encode_cpe(&self) -> BitVec {
        // Match on the element type.
        match self {
            CallElement::U8(u8_value) => {
                // Get the u8 value.
                let value = *u8_value;

                // Convert value to bytes.
                let byte: [u8; 1] = value.to_le_bytes();

                // Convert bytes to bits.
                let bits = BitVec::from_bytes(&byte);

                // Return the bits.
                bits
            }
            CallElement::U16(u16_value) => {
                // Get the u16 value.
                let value = *u16_value;

                // Convert value to bytes.
                let bytes: [u8; 2] = value.to_le_bytes();

                // Convert bytes to bits.
                let bits = BitVec::from_bytes(&bytes);

                // Return the bits.
                bits
            }
            CallElement::U32(short_val) => {
                // Encode the `ShortVal`.
                let bits = short_val.encode_cpe();

                // Return the bits.
                bits
            }
            CallElement::U64(long_val) => {
                // Encode the `LongVal`.
                let bits = long_val.encode_cpe();

                // Return the bits.
                bits
            }
            CallElement::Bool(value) => {
                // Get the bool value.
                let bool = *value;

                // Initialize bit vector.
                let mut bits = BitVec::new();

                // Push the bool value.
                bits.push(bool);

                // Return the bits.
                bits
            }
            CallElement::Account(account) => {
                // Encode the `Account`.
                let bits = account.encode_cpe();

                // Return the bits.
                bits
            }
            CallElement::Contract(contract) => {
                // Encode the `Contract`.
                let bits = contract.encode_cpe();

                // Return the bits.
                bits
            }
            CallElement::Bytes(bytes) => {
                // Encode the bytes.
                let bits = BitVec::from_bytes(bytes);

                // Return the bits.
                bits
            }
            CallElement::Varbytes(bytes) => {
                // Initialize bit vector to fill with length plus data.
                let mut bits = BitVec::new();

                // Get the byte length value.
                let byte_length = bytes.len() as u16;

                // Byte length as 2 bytes.
                let byte_length_bits = convert_u16_to_12_bits(byte_length);

                // Extend the bit vector with the byte length.
                bits.extend(byte_length_bits);

                // If data length is 0, return the bit vector with length-bits-only.
                // This is to avoid encoding empty data, as data can be empty.
                if byte_length == 0 {
                    return bits;
                }

                // Get the data bits.
                let data_bits = BitVec::from_bytes(bytes);

                // Extend the bit vector with the data bits.
                bits.extend(data_bits);

                // Return the bits.
                bits
            }
            CallElement::Payable(short_val) => {
                // Encode the `ShortVal`.
                let bits = short_val.encode_cpe();

                // Return the bits.
                bits
            }
        }
    }
}

/// Converts a u16 to 12 bits.
fn convert_u16_to_12_bits(value: u16) -> BitVec {
    // Byte length as 2 bytes.
    let byte_length_bytes = value.to_le_bytes();

    // Initialize byte length bits.
    let mut byte_length_bits = BitVec::new();

    // Convert byte length to bits.
    for i in 0..12 {
        let byte_idx = i / 8;
        let bit_idx = i % 8;
        byte_length_bits.push((byte_length_bytes[byte_idx] >> bit_idx) & 1 == 1);
    }

    // Return the bits.
    byte_length_bits
}
