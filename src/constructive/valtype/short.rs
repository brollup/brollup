use crate::cpe::CommonIntCPEDecodingError;
use crate::cpe::{CPEDecodingError, CompactPayloadEncoding, ShortValCPEDecodingError};
use async_trait::async_trait;
use bit_vec::BitVec;
use bit_vec::Iter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShortVal(pub u32);

/// Represents the tier of an uncommon `ShortVal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UncommonShortValTier {
    U8,
    U16,
    U24,
    U32,
}

impl ShortVal {
    /// Creates a new `ShortVal`
    pub fn new(short_val: u32) -> Self {
        Self(short_val)
    }

    /// Returns the core u32 value.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Determines the tier based on the value range.
    pub fn uncommon_tier(&self) -> UncommonShortValTier {
        match self.value() {
            0..=255 => UncommonShortValTier::U8,
            256..=65535 => UncommonShortValTier::U16,
            65536..=16777215 => UncommonShortValTier::U24,
            16777216..=4294967295 => UncommonShortValTier::U32,
        }
    }

    /// Returns the compact byte representation of the value if it is uncommon.
    pub fn compact_bytes(&self) -> Vec<u8> {
        let value = self.value();

        match self.uncommon_tier() {
            // 1 byte
            UncommonShortValTier::U8 => vec![value as u8],
            // 2 bytes
            UncommonShortValTier::U16 => {
                let bytes = value.to_le_bytes();
                vec![bytes[0], bytes[1]]
            }
            // 3 bytes
            UncommonShortValTier::U24 => {
                let bytes = value.to_le_bytes();
                vec![bytes[0], bytes[1], bytes[2]]
            }
            // 4 bytes
            UncommonShortValTier::U32 => value.to_le_bytes().to_vec(),
        }
    }

    /// Constructs a `ShortVal` from its compact byte representation.
    pub fn from_compact_bytes(bytes: &[u8]) -> Option<Self> {
        let value = match bytes.len() {
            // 1 byte (u8)
            1 => bytes[0] as u32,
            // 2 bytes (u16)
            2 => u16::from_le_bytes([bytes[0], bytes[1]]) as u32,
            // 3 bytes (u24)
            3 => {
                let mut buf = [0u8; 4];
                buf[..3].copy_from_slice(bytes);
                u32::from_le_bytes(buf)
            }
            // 4 bytes (u32)
            4 => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            _ => return None,
        };

        Some(Self(value))
    }

    /// Compact payload decoding for `ShortVal`.
    /// Decodes a `ShortVal` from a bit stream.
    pub fn decode_cpe(bit_stream: &mut bit_vec::Iter<'_>) -> Result<ShortVal, CPEDecodingError> {
        // Check if the value is common.
        let is_common = bit_stream
            .next()
            .ok_or(CPEDecodingError::ShortValCPEDecodingError(
                ShortValCPEDecodingError::BitStreamIteratorError,
            ))?;

        // Match the value.
        match is_common {
            true => {
                // Decode the common integer.
                let common_val = CommonShortVal::decode_cpe(bit_stream).map_err(|_| {
                    CPEDecodingError::ShortValCPEDecodingError(
                        ShortValCPEDecodingError::CommonIntDecodingError,
                    )
                })?;

                // Return the `ShortVal`.
                Ok(ShortVal::new(common_val))
            }
            false => {
                // Decode the uncommon tier.
                let tier = match (bit_stream.next(), bit_stream.next()) {
                    (Some(false), Some(false)) => UncommonShortValTier::U8,
                    (Some(false), Some(true)) => UncommonShortValTier::U16,
                    (Some(true), Some(false)) => UncommonShortValTier::U24,
                    (Some(true), Some(true)) => UncommonShortValTier::U32,
                    _ => {
                        return Err(CPEDecodingError::ShortValCPEDecodingError(
                            ShortValCPEDecodingError::BitStreamIteratorError,
                        ))
                    }
                };

                // Get the bit count for the tier.
                let bit_count = match tier {
                    UncommonShortValTier::U8 => 8,
                    UncommonShortValTier::U16 => 16,
                    UncommonShortValTier::U24 => 24,
                    UncommonShortValTier::U32 => 32,
                };

                // Collect the value bits.
                let mut value_bits = BitVec::new();
                for _ in 0..bit_count {
                    value_bits.push(bit_stream.next().ok_or(
                        CPEDecodingError::ShortValCPEDecodingError(
                            ShortValCPEDecodingError::BitStreamIteratorError,
                        ),
                    )?);
                }

                // Convert the value bits to bytes.
                let value_bytes = value_bits.to_bytes();

                // Construct the short value.
                let short_val = ShortVal::from_compact_bytes(&value_bytes).ok_or(
                    CPEDecodingError::ShortValCPEDecodingError(
                        ShortValCPEDecodingError::ShortValConversionError,
                    ),
                )?;

                // Return the `ShortVal`.
                Ok(short_val)
            }
        }
    }
}

#[async_trait]
impl CompactPayloadEncoding for ShortVal {
    fn encode_cpe(&self) -> BitVec {
        let mut bits = BitVec::new();

        // Try to encode self value as a common integer.
        match CommonShortVal::encode(self.value()) {
            Some(common_int_bits) => {
                // Push true for the common integer.
                bits.push(true);

                // Push the common integer bits.
                bits.extend(common_int_bits);

                // Return the bits.
                return bits;
            }
            None => {
                // Push false for the uncommon integer.
                bits.push(false);

                // Fill with tier bits.
                match self.uncommon_tier() {
                    // 00 for u8
                    UncommonShortValTier::U8 => {
                        bits.push(false);
                        bits.push(false);
                    }
                    // 01 for u16
                    UncommonShortValTier::U16 => {
                        bits.push(false);
                        bits.push(true);
                    }
                    // 10 for u24
                    UncommonShortValTier::U24 => {
                        bits.push(true);
                        bits.push(false);
                    }
                    // 11 for u32
                    UncommonShortValTier::U32 => {
                        bits.push(true);
                        bits.push(true);
                    }
                }

                // Fill with value bits.
                let value_bits = BitVec::from_bytes(&self.compact_bytes());
                bits.extend(value_bits);

                // Return the bits.
                bits
            }
        }
    }
}

/// A list of 64 common u32 integers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommonShortVal {
    // Enum variants
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
    Fourteen,
    Fifteen,
    Eighteen,
    Twenty,
    TwentyFive,
    TwentyEight,
    Thirty,
    ThirtyFive,
    Forty,
    FortyFive,
    Fifty,
    FiftyFive,
    Sixty,
    Seventy,
    SeventyFive,
    Eighty,
    Ninety,
    Hundred,
    HundredFifty,
    TwoHundred,
    TwoFifty,
    ThreeHundred,
    FourHundred,
    FiveHundred,
    FiveFifty,
    SixHundred,
    SevenFifty,
    EightHundred,
    NineHundred,
    Thousand,
    ThousandFiveHundred,
    TwoThousand,
    TwoThousandFiveHundred,
    ThreeThousand,
    FourThousand,
    FiveThousand,
    FiveThousandFiveHundred,
    SevenThousandFiveHundred,
    TenThousand,
    ThirtyFiveThousand,
    TwentyFiveThousand,
    FiftyThousand,
    SeventyFiveThousand,
    HundredThousand,
    TwoHundredFiftyThousand,
    FiveHundredThousand,
    Million,
    TwoMillion,
    FiveMillion,
    TenMillion,
    TwentyFiveMillion,
    FiftyMillion,
    HundredMillion,
    Billion,
}

impl CommonShortVal {
    /// Encodes a `u32` integer into a 6-bit `bit_vec::BitVec`.
    pub fn encode(value: u32) -> Option<BitVec> {
        let index = match value {
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

        // Create a 6-bit `BitVec` based on the index
        let mut bv = BitVec::from_elem(6, false);
        for bit in 0..6 {
            bv.set(bit, (index & (1 << bit)) != 0);
        }
        Some(bv)
    }

    /// Compact payload encoding for `CommonInt`.
    /// Decodes a common u32 integer from a 6-bit `bit_vec::Iter`.
    pub fn decode_cpe(bit_stream: &mut Iter<'_>) -> Result<u32, CommonIntCPEDecodingError> {
        let mut index: u8 = 0;

        // Collect 6 bits from the bit stream
        for i in 0..6 {
            match bit_stream.next() {
                Some(true) => index |= 1 << i,
                Some(false) => {}
                None => return Err(CommonIntCPEDecodingError::BitStreamIteratorError),
            }
        }

        // Convert the 6-bit index to a CommonInt variant and its u32 value
        match index {
            0 => Ok(1),
            1 => Ok(2),
            2 => Ok(3),
            3 => Ok(4),
            4 => Ok(5),
            5 => Ok(6),
            6 => Ok(7),
            7 => Ok(8),
            8 => Ok(9),
            9 => Ok(10),
            10 => Ok(14),
            11 => Ok(15),
            12 => Ok(18),
            13 => Ok(20),
            14 => Ok(25),
            15 => Ok(28),
            16 => Ok(30),
            17 => Ok(35),
            18 => Ok(40),
            19 => Ok(45),
            20 => Ok(50),
            21 => Ok(55),
            22 => Ok(60),
            23 => Ok(70),
            24 => Ok(75),
            25 => Ok(80),
            26 => Ok(90),
            27 => Ok(100),
            28 => Ok(150),
            29 => Ok(200),
            30 => Ok(250),
            31 => Ok(300),
            32 => Ok(400),
            33 => Ok(500),
            34 => Ok(550),
            35 => Ok(600),
            36 => Ok(750),
            37 => Ok(800),
            38 => Ok(900),
            39 => Ok(1_000),
            40 => Ok(1_500),
            41 => Ok(2_000),
            42 => Ok(2_500),
            43 => Ok(3_000),
            44 => Ok(4_000),
            45 => Ok(5_000),
            46 => Ok(5_500),
            47 => Ok(7_500),
            48 => Ok(10_000),
            49 => Ok(25_000),
            50 => Ok(35_000),
            51 => Ok(50_000),
            52 => Ok(75_000),
            53 => Ok(100_000),
            54 => Ok(250_000),
            55 => Ok(500_000),
            56 => Ok(1_000_000),
            57 => Ok(2_000_000),
            58 => Ok(5_000_000),
            59 => Ok(10_000_000),
            60 => Ok(25_000_000),
            61 => Ok(50_000_000),
            62 => Ok(100_000_000),
            63 => Ok(1_000_000_000),
            _ => Err(CommonIntCPEDecodingError::UncommonInteger),
        }
    }
}
