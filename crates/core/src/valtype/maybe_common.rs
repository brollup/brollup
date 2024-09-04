#![allow(dead_code)]

use bit_vec::BitVec;

use crate::encoding::{cpe::CommonIndex, cpe::CompactPayloadEncoding};
use std::u8;

pub trait MaybeCommonType {}

impl MaybeCommonType for super::account::Account {}
impl MaybeCommonType for super::contract::Contract {}
impl MaybeCommonType for super::value::ShortVal {}
impl MaybeCommonType for super::value::LongVal {}

#[derive(Clone, Copy)]
pub enum MaybeCommon<T: MaybeCommonType> {
    Common(T, u8),
    Uncommon(T),
}

impl<T: MaybeCommonType + CompactPayloadEncoding> CompactPayloadEncoding for MaybeCommon<T> {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        match self {
            MaybeCommon::Uncommon(uncommon) => {
                // Common bit = false
                bit_vec.push(false);
                // Bit-encoding:
                bit_vec.extend(uncommon.to_cpe());
                bit_vec
            }
            MaybeCommon::Common(_, common_index) => {
                // Common bit = true
                bit_vec.push(true);
                // 3-bit common index encoding:
                bit_vec.extend(BitVec::from_u8_common_index(common_index));
                bit_vec
            }
        }
    }
}