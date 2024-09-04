#![allow(dead_code)]

use crate::encoding::cpe::CompactPayloadEncoding;
use bit_vec::BitVec;
use uintx::{u24, u40, u48, u56};

#[derive(Clone, Copy)]
pub struct ShortVal(pub u32);

#[derive(Clone, Copy)]
pub struct LongVal(pub u64);

impl ShortVal {
    pub fn new(value: u32) -> ShortVal {
        ShortVal(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl LongVal {
    pub fn new(value: u64) -> LongVal {
        LongVal(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl CompactPayloadEncoding for ShortVal {
    fn to_cpe(&self) -> BitVec {
        let value = self.0;
        let mut bit_vec = BitVec::new();

        match value {
            0..=255 => {
                // b00 -> UInt 8 (1-byte)
                bit_vec.push(false);
                bit_vec.push(false);

                let val_byte = vec![value as u8];
                let val_bits = BitVec::from_bytes(&val_byte);
                bit_vec.extend(val_bits);
            }

            256..=65535 => {
                // b01 -> UInt 16 (2 bytes)
                bit_vec.push(false);
                bit_vec.push(true);

                let val_bytes: [u8; 2] = (value as u16).to_le_bytes();
                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }

            65536..=16777215 => {
                // b10 -> UInt 24 (3 bytes)
                bit_vec.push(true);
                bit_vec.push(false);

                let val_bytes: [u8; 3] = u24::from(value).to_le_bytes();
                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }

            16777216..=4294967295 => {
                // b11 -> UInt 32 (4 bytes)
                bit_vec.push(true);
                bit_vec.push(true);

                let val_bytes: [u8; 4] = (value as u32).to_le_bytes();
                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }
        }

        bit_vec
    }
}

impl CompactPayloadEncoding for LongVal {
    fn to_cpe(&self) -> BitVec {
        let value = self.0;
        let mut bit_vec = BitVec::new();

        match value {
            0..=4294967295 => {
                // Interpet as Short Val and cast to Long Val by appending a zero-bit prefix
                bit_vec.push(false);
                bit_vec.extend(ShortVal(value as u32).to_cpe());
            }

            4294967296..=1099511627775 => {
                // b100 -> UInt 40 (5 bytes)
                bit_vec.push(true);
                bit_vec.push(false);
                bit_vec.push(false);

                let val_bytes: [u8; 5] = u40::from(value).to_le_bytes();
                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }

            1099511627776..=281474976710655 => {
                // b101 -> UInt 48 (6 bytes)
                bit_vec.push(true);
                bit_vec.push(false);
                bit_vec.push(true);

                let val_bytes: [u8; 6] = u48::from(value).to_le_bytes();
                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }

            281474976710656..=72057594037927935 => {
                // b110 -> UInt 56 (7 bytes)
                bit_vec.push(true);
                bit_vec.push(true);
                bit_vec.push(false);

                let val_bytes: [u8; 7] = u56::from(value).to_le_bytes();
                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }

            72057594037927936..=18446744073709551615 => {
                // b111 -> UInt 64 (8 bytes)
                bit_vec.push(true);
                bit_vec.push(true);
                bit_vec.push(true);

                let val_bytes: [u8; 8] = value.to_le_bytes();
                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }
        }

        bit_vec
    }
}
