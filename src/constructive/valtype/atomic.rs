use crate::cpe::{CPEError, CompactPayloadEncoding};
use async_trait::async_trait;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// Atomic compact value representation from zero to seven.
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
}

impl AtomicVal {
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
    pub fn value_as_u8(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
        }
    }

    /// Compact payload decoding for `AtomicVal`.
    /// Decodes an `AtomicVal` from a bit stream and returns it along with the remaining bit stream.
    pub fn decode_cpe(
        mut bit_stream: bit_vec::Iter<'_>,
    ) -> Result<(AtomicVal, bit_vec::Iter<'_>), CPEError> {
        // Decode the first 3 bits
        let value = match (bit_stream.next(), bit_stream.next(), bit_stream.next()) {
            // 000 for 0
            (Some(false), Some(false), Some(false)) => Self::Zero,
            // 001 for 1
            (Some(false), Some(false), Some(true)) => Self::One,
            // 010 for 2
            (Some(false), Some(true), Some(false)) => Self::Two,
            // 011 for 3
            (Some(false), Some(true), Some(true)) => Self::Three,
            // 100 for 4
            (Some(true), Some(false), Some(false)) => Self::Four,
            // 101 for 5
            (Some(true), Some(false), Some(true)) => Self::Five,
            // 110 for 6
            (Some(true), Some(true), Some(false)) => Self::Six,
            // 111 for 7
            (Some(true), Some(true), Some(true)) => Self::Seven,
            _ => return Err(CPEError::IteratorError),
        };

        // Return the decoded value and the remaining bit stream.
        Ok((value, bit_stream))
    }
}

#[async_trait]
impl CompactPayloadEncoding for AtomicVal {
    fn encode_cpe(&self) -> BitVec {
        let mut bits = BitVec::new();

        match self {
            Self::Zero => {
                // 000 for 0
                bits.push(false);
                bits.push(false);
                bits.push(false);
            }
            Self::One => {
                // 001 for 1
                bits.push(false);
                bits.push(false);
                bits.push(true);
            }
            Self::Two => {
                // 010 for 2
                bits.push(false);
                bits.push(true);
                bits.push(false);
            }
            Self::Three => {
                // 011 for 3
                bits.push(false);
                bits.push(true);
                bits.push(true);
            }
            Self::Four => {
                // 100 for 4
                bits.push(true);
                bits.push(false);
                bits.push(false);
            }
            Self::Five => {
                // 101 for 5
                bits.push(true);
                bits.push(false);
                bits.push(true);
            }
            Self::Six => {
                // 110 for 6
                bits.push(true);
                bits.push(true);
                bits.push(false);
            }
            Self::Seven => {
                // 111 for 7
                bits.push(true);
                bits.push(true);
                bits.push(true);
            }
        }

        bits
    }
}
