use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

use crate::cpe::{CPEDecodingError, CompactPayloadEncoding, MaybeCommonCPEDecodingError};

/// `CommonVal` represents a common integer value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonVal(pub u32, pub u8);

impl CommonVal {
    /// Creates a new `CommonVal` from a `u32` integer.
    pub fn new(value: u32) -> Option<CommonVal> {
        // Get the bitcode.
        let bitcode = match value {
            1 => 0,
            2 => 1,
            3 => 2,
            4 => 3,
            5 => 4,
            6 => 5,
            7 => 6,
            8 => 7,
            9 => 8,
            10 => 9,
            14 => 10,
            15 => 11,
            18 => 12,
            20 => 13,
            25 => 14,
            28 => 15,
            30 => 16,
            35 => 17,
            40 => 18,
            45 => 19,
            50 => 20,
            55 => 21,
            60 => 22,
            70 => 23,
            75 => 24,
            80 => 25,
            90 => 26,
            100 => 27,
            150 => 28,
            200 => 29,
            250 => 30,
            300 => 31,
            400 => 32,
            500 => 33,
            550 => 34,
            600 => 35,
            750 => 36,
            800 => 37,
            900 => 38,
            1_000 => 39,
            1_500 => 40,
            2_000 => 41,
            2_500 => 42,
            3_000 => 43,
            4_000 => 44,
            5_000 => 45,
            5_500 => 46,
            7_500 => 47,
            10_000 => 48,
            25_000 => 49,
            35_000 => 50,
            50_000 => 51,
            75_000 => 52,
            100_000 => 53,
            250_000 => 54,
            500_000 => 55,
            1_000_000 => 56,
            2_000_000 => 57,
            5_000_000 => 58,
            10_000_000 => 59,
            25_000_000 => 60,
            50_000_000 => 61,
            100_000_000 => 62,
            1_000_000_000 => 63,
            _ => return None, // Invalid value
        };

        // Create a new `CommonVal`.
        let common_short_val = CommonVal(value, bitcode);

        // Return the `CommonVal`.
        Some(common_short_val)
    }

    /// Returns the inner u32 value of the `CommonVal`.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Returns the bitcode of the `CommonVal`.
    pub fn bitcode(&self) -> u8 {
        self.1
    }

    /// Compact payload encoding for `CommonInt`.
    /// Decodes a common u32 integer from a 6-bit `bit_vec::Iter`.
    pub fn decode_cpe(bit_stream: &mut bit_vec::Iter<'_>) -> Result<Self, CPEDecodingError> {
        let mut bitcode: u8 = 0;

        // Collect 6 bits from the bit stream
        for i in 0..6 {
            match bit_stream.next() {
                Some(true) => bitcode |= 1 << i,
                Some(false) => {}
                None => {
                    return Err(CPEDecodingError::MaybeCommonCPEDecodingError(
                        MaybeCommonCPEDecodingError::BitStreamIteratorError,
                    ))
                }
            }
        }

        // Convert the 6-bit bitcode to a CommonInt variant and its u32 value
        let value: u32 = match bitcode {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
            5 => 6,
            6 => 7,
            7 => 8,
            8 => 9,
            9 => 10,
            10 => 14,
            11 => 15,
            12 => 18,
            13 => 20,
            14 => 25,
            15 => 28,
            16 => 30,
            17 => 35,
            18 => 40,
            19 => 45,
            20 => 50,
            21 => 55,
            22 => 60,
            23 => 70,
            24 => 75,
            25 => 80,
            26 => 90,
            27 => 100,
            28 => 150,
            29 => 200,
            30 => 250,
            31 => 300,
            32 => 400,
            33 => 500,
            34 => 550,
            35 => 600,
            36 => 750,
            37 => 800,
            38 => 900,
            39 => 1_000,
            40 => 1_500,
            41 => 2_000,
            42 => 2_500,
            43 => 3_000,
            44 => 4_000,
            45 => 5_000,
            46 => 5_500,
            47 => 7_500,
            48 => 10_000,
            49 => 25_000,
            50 => 35_000,
            51 => 50_000,
            52 => 75_000,
            53 => 100_000,
            54 => 250_000,
            55 => 500_000,
            56 => 1_000_000,
            57 => 2_000_000,
            58 => 5_000_000,
            59 => 10_000_000,
            60 => 25_000_000,
            61 => 50_000_000,
            62 => 100_000_000,
            63 => 1_000_000_000,
            _ => {
                return Err(CPEDecodingError::MaybeCommonCPEDecodingError(
                    MaybeCommonCPEDecodingError::UncommonInteger,
                ))
            }
        };

        // Create a new `CommonVal`.
        let common_short_val = CommonVal(value, bitcode);

        // Return the `CommonVal`.
        Ok(common_short_val)
    }
}

impl CompactPayloadEncoding for CommonVal {
    fn encode_cpe(&self) -> BitVec {
        // Get the bitcode.
        let bitcode = self.bitcode();

        // Create a 6-bit `BitVec` based on the bitcode.
        let mut bits = BitVec::from_elem(6, false);
        for bit in 0..6 {
            bits.set(bit, (bitcode & (1 << bit)) != 0);
        }

        // Return the bits.
        bits
    }
}
