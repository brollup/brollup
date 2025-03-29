use crate::cpe::{
    cpe::CompactPayloadEncoding,
    decode_error::{error::CPEDecodingError, valtype_error::AtomicValCPEDecodingError},
};
use async_trait::async_trait;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// Atomic compact value representation from zero to fifteen.
///
/// Used for very small value representations such as contract method call indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AtomicVal {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

impl AtomicVal {
    pub fn new_u8(value: u8) -> Option<AtomicVal> {
        match value {
            0 => Some(AtomicVal::Zero),
            1 => Some(AtomicVal::One),
            2 => Some(AtomicVal::Two),
            3 => Some(AtomicVal::Three),
            4 => Some(AtomicVal::Four),
            5 => Some(AtomicVal::Five),
            6 => Some(AtomicVal::Six),
            7 => Some(AtomicVal::Seven),
            8 => Some(AtomicVal::Eight),
            9 => Some(AtomicVal::Nine),
            10 => Some(AtomicVal::Ten),
            11 => Some(AtomicVal::Eleven),
            12 => Some(AtomicVal::Twelve),
            13 => Some(AtomicVal::Thirteen),
            14 => Some(AtomicVal::Fourteen),
            15 => Some(AtomicVal::Fifteen),
            _ => None,
        }
    }
    /// Returns the atomic value zero.
    pub fn zero() -> Self {
        Self::Zero
    }

    /// Returns the atomic value one.
    pub fn one() -> Self {
        Self::One
    }

    /// Returns the atomic value two.
    pub fn two() -> Self {
        Self::Two
    }

    /// Returns the atomic value three.
    pub fn three() -> Self {
        Self::Three
    }

    /// Returns the atomic value four.
    pub fn four() -> Self {
        Self::Four
    }

    /// Returns the atomic value five.
    pub fn five() -> Self {
        Self::Five
    }

    /// Returns the atomic value six.
    pub fn six() -> Self {
        Self::Six
    }

    /// Returns the atomic value seven.
    pub fn seven() -> Self {
        Self::Seven
    }

    /// Returns the value as a u8.
    pub fn value(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Eleven => 11,
            Self::Twelve => 12,
            Self::Thirteen => 13,
            Self::Fourteen => 14,
            Self::Fifteen => 15,
        }
    }

    /// Compact payload decoding for `AtomicVal`.
    /// Decodes an `AtomicVal` from a bit stream.
    pub fn decode_cpe(bit_stream: &mut bit_vec::Iter<'_>) -> Result<AtomicVal, CPEDecodingError> {
        // Initialize a BitVec.
        let mut bits = BitVec::new();

        // Collect the 4 bits four times iteratively.
        for _ in 0..4 {
            bits.push(
                bit_stream
                    .next()
                    .ok_or(CPEDecodingError::AtomicValCPEDecodingError(
                        AtomicValCPEDecodingError::BitStreamIteratorError,
                    ))?,
            );
        }

        // Convert the 4 bits to a u8 value.
        let value = convert_4_bits_to_u8(&bits);

        // Convert the u8 value to an `AtomicVal`.
        let atomic_val =
            AtomicVal::new_u8(value).ok_or(CPEDecodingError::AtomicValCPEDecodingError(
                AtomicValCPEDecodingError::AtomicValConversionError,
            ))?;

        // Return the `AtomicVal`.
        Ok(atomic_val)
    }
}

#[async_trait]
impl CompactPayloadEncoding for AtomicVal {
    fn encode_cpe(&self) -> Option<BitVec> {
        // Convert the value to a 4-bit BitVec.
        let bits = convert_u8_to_4_bits(self.value());

        // Return the BitVec.
        Some(bits)
    }
}

/// Convert a u8 to a 4-bit BitVec.
fn convert_u8_to_4_bits(value: u8) -> BitVec {
    let val_bytes = value.to_le_bytes();

    // Initialize a BitVec.
    let mut val_bits = BitVec::new();

    // Iterate 4 times to push the 4 bits.
    for i in 0..4 {
        val_bits.push((val_bytes[0] >> i) & 1 == 1);
    }

    // Return the BitVec.
    val_bits
}

/// Convert a 4-bit BitVec to a u8.
fn convert_4_bits_to_u8(bits: &BitVec) -> u8 {
    // Initialize a u8.
    let mut decoded_val = 0u8;

    // Iterate 4 times to decode the 4 bits.
    for i in 0..4 {
        let bit = bits[i];
        if bit {
            decoded_val |= 1 << i;
        }
    }

    // Return the u8.
    decoded_val
}
