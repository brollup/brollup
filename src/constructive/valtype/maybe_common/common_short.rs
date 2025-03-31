use crate::constructive::cpe::{
    cpe::CompactPayloadEncoding,
    decode_error::{
        error::CPEDecodingError,
        valtype_error::{CommonShortValCPEDecodingError, MaybeCommonCPEDecodingError},
    },
};
use crate::constructive::valtype::u8_ext::U8Ext;
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

/// The bitsize of the `CommonShortVal`.
const COMMON_SHORT_BITSIZE: u8 = 6;

/// `CommonShortVal` represents a common u32 integer value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonShortVal {
    value: u32,
    index: u8,
}

impl CommonShortVal {
    /// Creates a new `CommonShortVal` from a `u32` integer.
    pub fn new(value: u32) -> Option<CommonShortVal> {
        // Get the index.
        let index = match value {
            50 => 0,
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

        // Create a new `CommonShortVal`.
        let common_short_val = CommonShortVal { value, index };

        // Return the `CommonShortVal`.
        Some(common_short_val)
    }

    /// Returns the inner u32 value of the `CommonShortVal`.
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Returns the index of the `CommonShortVal`.
    pub fn index(&self) -> u8 {
        self.index
    }

    /// Compact payload encoding for `CommonShortVal`.
    /// Decodes a common u32 integer from a 6-bit `bit_vec::Iter`.
    pub fn decode_cpe(bit_stream: &mut bit_vec::Iter<'_>) -> Result<Self, CPEDecodingError> {
        // Initialize empty bitvec.
        let mut bits = BitVec::new();

        // Collect 6 bits from the bit stream
        for _ in 0..COMMON_SHORT_BITSIZE {
            let bit = bit_stream
                .next()
                .ok_or(CPEDecodingError::MaybeCommonCPEDecodingError(
                    MaybeCommonCPEDecodingError::CommonShortValCPEDecodingError(
                        CommonShortValCPEDecodingError::BitStreamIteratorError,
                    ),
                ))?;

            // Push the bit to the bitvec.
            bits.push(bit);
        }

        // Decode index.
        let index = u8::from_bits(&bits, COMMON_SHORT_BITSIZE).ok_or(
            CPEDecodingError::MaybeCommonCPEDecodingError(
                MaybeCommonCPEDecodingError::CommonShortValCPEDecodingError(
                    CommonShortValCPEDecodingError::U8BitCodecError,
                ),
            ),
        )?;

        // Convert the 6-bit index to a CommonInt variant and its u32 value
        let value: u32 = match index {
            0 => 50,
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
                    MaybeCommonCPEDecodingError::CommonShortValCPEDecodingError(
                        CommonShortValCPEDecodingError::UncommonInteger,
                    ),
                ))
            }
        };

        // Create a new `CommonVal`.
        let common_short_val = CommonShortVal { value, index };

        // Return the `CommonVal`.
        Ok(common_short_val)
    }
}

impl CompactPayloadEncoding for CommonShortVal {
    fn encode_cpe(&self) -> Option<BitVec> {
        // Get the index.
        let index = self.index();

        // Convert index to a bits with the bitsize of 6.
        let bits = u8::to_bits(index, COMMON_SHORT_BITSIZE)?;

        // Return the bits.
        Some(bits)
    }
}
