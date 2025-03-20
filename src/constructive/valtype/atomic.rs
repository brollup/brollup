use crate::cpe::CompactPayloadEncoding;
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
}

impl CompactPayloadEncoding for AtomicVal {
    fn encode(&self) -> BitVec {
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

    fn decode(bits: &BitVec) -> Option<(Self, BitVec)> {
        let mut iter = bits.iter();

        let value = match (iter.next()?, iter.next()?, iter.next()?) {
            // 000 for 0
            (false, false, false) => Self::Zero,
            // 001 for 1
            (false, false, true) => Self::One,
            // 010 for 2
            (false, true, false) => Self::Two,
            // 011 for 3
            (false, true, true) => Self::Three,
            // 100 for 4
            (true, false, false) => Self::Four,
            // 101 for 5
            (true, false, true) => Self::Five,
            // 110 for 6
            (true, true, false) => Self::Six,
            // 111 for 7
            (true, true, true) => Self::Seven,
        };

        let remaining_bits = iter.collect::<BitVec>();

        Some((value, remaining_bits))
    }
}
