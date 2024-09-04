#![allow(dead_code)]

use super::value::ShortVal;
use crate::encoding::cpe::CompactPayloadEncoding;
use bit_vec::BitVec;
use musig2::secp256k1::XOnlyPublicKey;

type Key = XOnlyPublicKey;

#[derive(Clone, Copy)]
pub struct Account {
    key: Key,
    account_index: Option<u32>,
}

impl Account {
    pub fn new(key: Key) -> Account {
        Account {
            key,
            account_index: None,
        }
    }

    pub fn new_compact(key: Key, account_index: u32) -> Account {
        Account {
            key,
            account_index: Some(account_index),
        }
    }

    pub fn key(&self) -> Key {
        self.key
    }

    pub fn account_index(&self) -> Option<u32> {
        self.account_index
    }

    pub fn set_account_index(&mut self, account_index: u32) {
        self.account_index = Some(account_index);
    }
}

impl CompactPayloadEncoding for Account {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        match self.account_index {
            None => {
                // Non-compact form
                bit_vec.push(false);

                let key_array = self.key.serialize();
                let key_bits = BitVec::from_bytes(&key_array);

                bit_vec.extend(key_bits);
            }
            Some(index) => {
                // Compact form
                bit_vec.push(true);

                // ShortVal represents compact integer forms
                let index_compact = ShortVal(index);

                bit_vec.extend(index_compact.to_cpe());
            }
        }

        bit_vec
    }
}
