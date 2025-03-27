use crate::cpe::decode_error::{
    error::CPEDecodingError, error::CompactPayloadEncoding,
    valtype_error::MaybeCommonCPEDecodingError,
};
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// `CommonVal` represents a common integer value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonVal {
    value: u32,
    index: u8,
}

impl CommonVal {
    /// Creates a new `CommonVal` from a `u32` integer.
    pub fn new(value: u32) -> Option<CommonVal> {
        // Get the index.
        let index = match value {
            100 => 1,
            200 => 2,
            250 => 3,
            500 => 4,
            550 => 5,
            600 => 6,
            750 => 7,
            800 => 8,
            900 => 9,
            999 => 10,
            1_000 => 11,
            1_500 => 12,
            2_000 => 13,
            2_500 => 14,
            3_000 => 15,
            4_000 => 16,
            5_000 => 17,
            5_500 => 18,
            6_000 => 19,
            7_000 => 20,
            7_500 => 21,
            8_000 => 22,
            9_000 => 23,
            9_900 => 24,
            10_000 => 25,
            15_000 => 26,
            20_000 => 27,
            25_000 => 28,
            30_000 => 29,
            35_000 => 30,
            40_000 => 31,
            50_000 => 32,
            55_000 => 33,
            60_000 => 34,
            70_000 => 35,
            75_000 => 36,
            80_000 => 37,
            90_000 => 38,
            99_900 => 39,
            100_000 => 40,
            150_000 => 41,
            200_000 => 42,
            250_000 => 43,
            300_000 => 44,
            350_000 => 45,
            400_000 => 46,
            500_000 => 47,
            550_000 => 48,
            600_000 => 49,
            700_000 => 50,
            750_000 => 51,
            800_000 => 52,
            900_000 => 53,
            1_000_000 => 54,
            2_000_000 => 55,
            2_500_000 => 56,
            5_000_000 => 57,
            5_500_000 => 58,
            10_000_000 => 59,
            25_000_000 => 60,
            50_000_000 => 61,
            75_000_000 => 62,
            100_000_000 => 63,

            _ => return None, // Invalid value
        };

        // Create a new `CommonVal`.
        let common_short_val = CommonVal { value, index };

        // Return the `CommonVal`.
        Some(common_short_val)
    }

    /// Returns the inner u32 value of the `CommonVal`.
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Returns the index of the `CommonVal`.
    pub fn index(&self) -> u8 {
        self.index
    }

    /// Compact payload encoding for `CommonInt`.
    /// Decodes a common u32 integer from a 6-bit `bit_vec::Iter`.
    pub fn decode_cpe(bit_stream: &mut bit_vec::Iter<'_>) -> Result<Self, CPEDecodingError> {
        let mut index: u8 = 0;

        // Collect 6 bits from the bit stream
        for i in 0..6 {
            match bit_stream.next() {
                Some(true) => index |= 1 << i,
                Some(false) => {}
                None => {
                    return Err(CPEDecodingError::MaybeCommonCPEDecodingError(
                        MaybeCommonCPEDecodingError::BitStreamIteratorError,
                    ))
                }
            }
        }

        // Convert the 6-bit index to a CommonInt variant and its u32 value
        let value: u32 = match index {
            1 => 100,
            2 => 200,
            3 => 250,
            4 => 500,
            5 => 550,
            6 => 600,
            7 => 750,
            8 => 800,
            9 => 900,
            10 => 999,
            11 => 1_000,
            12 => 1_500,
            13 => 2_000,
            14 => 2_500,
            15 => 3_000,
            16 => 4_000,
            17 => 5_000,
            18 => 5_500,
            19 => 6_000,
            20 => 7_000,
            21 => 7_500,
            22 => 8_000,
            23 => 9_000,
            24 => 9_900,
            25 => 10_000,
            26 => 15_000,
            27 => 20_000,
            28 => 25_000,
            29 => 30_000,
            30 => 35_000,
            31 => 40_000,
            32 => 50_000,
            33 => 55_000,
            34 => 60_000,
            35 => 70_000,
            36 => 75_000,
            37 => 80_000,
            38 => 90_000,
            39 => 99_900,
            40 => 100_000,
            41 => 150_000,
            42 => 200_000,
            43 => 250_000,
            44 => 300_000,
            45 => 350_000,
            46 => 400_000,
            47 => 500_000,
            48 => 550_000,
            49 => 600_000,
            50 => 700_000,
            51 => 750_000,
            52 => 800_000,
            53 => 900_000,
            54 => 1_000_000,
            55 => 2_000_000,
            56 => 2_500_000,
            57 => 5_000_000,
            58 => 5_500_000,
            59 => 10_000_000,
            60 => 25_000_000,
            61 => 50_000_000,
            62 => 75_000_000,
            63 => 100_000_000,

            _ => {
                return Err(CPEDecodingError::MaybeCommonCPEDecodingError(
                    MaybeCommonCPEDecodingError::UncommonInteger,
                ))
            }
        };

        // Create a new `CommonVal`.
        let common_short_val = CommonVal { value, index };

        // Return the `CommonVal`.
        Ok(common_short_val)
    }
}

impl CompactPayloadEncoding for CommonVal {
    fn encode_cpe(&self) -> BitVec {
        // Get the index.
        let index = self.index();

        // Create a 6-bit `BitVec` based on the bitcode.
        let mut bits = BitVec::from_elem(6, false);
        for bit in 0..6 {
            bits.set(bit, (index & (1 << bit)) != 0);
        }

        // Return the bits.
        bits
    }
}
