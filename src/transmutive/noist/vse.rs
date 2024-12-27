use crate::hash::Hash;
use secp::{MaybePoint, MaybeScalar, Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn encrypting_key_secret(self_secret: Scalar, to_public: Point) -> Scalar {
    let shared_secret_point = self_secret * to_public;
    let shared_secret_point_xbytes = shared_secret_point.serialize_uncompressed();
    let shared_secret_point_hash = (&shared_secret_point_xbytes).hash();
    let shared_secret = match MaybeScalar::reduce_from(&shared_secret_point_hash) {
        MaybeScalar::Valid(scalar) => scalar,
        MaybeScalar::Zero => Scalar::reduce_from(&shared_secret_point_hash),
    };
    shared_secret
}

pub fn encrypting_key_public(self_secret: Scalar, to_public: Point) -> Point {
    encrypting_key_secret(self_secret, to_public).base_point_mul()
}

pub fn encrypt(secret_to_encrypt: Scalar, encrypting_key_secret: Scalar) -> Option<Scalar> {
    match secret_to_encrypt + encrypting_key_secret {
        MaybeScalar::Valid(scalar) => Some(scalar),
        MaybeScalar::Zero => None,
    }
}

pub fn decrypt(secret_to_decrypt: Scalar, encrypting_key_secret: Scalar) -> Option<Scalar> {
    match secret_to_decrypt - encrypting_key_secret {
        MaybeScalar::Valid(scalar) => Some(scalar),
        MaybeScalar::Zero => None,
    }
}

pub fn verify(
    encrypted_share_scalar: Scalar,
    public_share_point: Point,
    encrypting_key_public: Point,
) -> bool {
    let combined_point = encrypted_share_scalar.base_point_mul();

    combined_point
        == match public_share_point + encrypting_key_public {
            MaybePoint::Valid(point) => point,
            MaybePoint::Infinity => return false,
        }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct KeyMap {
    signer: [u8; 32],
    map: HashMap<[u8; 32], [u8; 32]>,
}

impl KeyMap {
    pub fn new(signer: [u8; 32]) -> KeyMap {
        KeyMap {
            signer,
            map: HashMap::<[u8; 32], [u8; 32]>::new(),
        }
    }

    pub fn signer_key(&self) -> [u8; 32] {
        self.signer
    }

    pub fn insert(&mut self, signer_key: [u8; 32], vse_key: [u8; 32]) {
        self.map.insert(signer_key, vse_key);
    }

    pub fn correspondant_signer_list(&self) -> Vec<[u8; 32]> {
        let mut keys: Vec<[u8; 32]> = self.map.keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn full_signer_list(&self) -> Vec<[u8; 32]> {
        let mut full_list = Vec::<[u8; 32]>::new();

        full_list.push(self.signer_key());
        full_list.extend(self.correspondant_signer_list());
        full_list.sort();

        full_list
    }

    pub fn is_complete(&self, expected_signer_list_: &Vec<[u8; 32]>) -> bool {
        let mut expected_signer_list = expected_signer_list_.clone();
        expected_signer_list.sort();

        let self_signer_list = self.full_signer_list();

        if self_signer_list.len() == expected_signer_list.len() {
            for (index, self_signer) in self_signer_list.iter().enumerate() {
                if self_signer.to_owned() != expected_signer_list[index] {
                    return false;
                }
            }
            return true;
        }

        false
    }

    pub fn vse_key(&self, correspondant: [u8; 32]) -> Option<[u8; 32]> {
        Some(self.map.get(&correspondant)?.to_owned())
    }
}

pub struct Directory {
    signers: Vec<[u8; 32]>,
    vse_keys: Vec<KeyMap>,
}

impl Directory {
    pub fn new(signers: &Vec<[u8; 32]>) -> Directory {
        Directory {
            signers: signers.clone(),
            vse_keys: Vec::<KeyMap>::new(),
        }
    }

    pub fn signers(&self) -> Vec<[u8; 32]> {
        self.signers.clone()
    }

    pub fn insert(&mut self, map: KeyMap) -> bool {
        if self.signers.contains(&map.signer_key()) {
            if map.is_complete(&self.signers()) {
                if !self.vse_keys.contains(&map) {
                    self.vse_keys.push(map);
                    return true;
                }
            }
        }
        false
    }

    pub fn map(&self, signer: [u8; 32]) -> Option<KeyMap> {
        for map in self.vse_keys.iter() {
            if map.signer_key() == signer {
                return Some(map.to_owned());
            }
        }

        None
    }

    pub fn is_complete(&self) -> bool {
        if self.vse_keys.len() != self.signers.len() {
            return false;
        }

        for map in self.vse_keys.iter() {
            if !map.is_complete(&self.signers()) {
                return false;
            }
        }

        true
    }

    pub fn validate(&self) -> bool {
        if !self.is_complete() {
            return false;
        }

        for signer in self.signers.iter() {
            let map = match self.map(signer.to_owned()) {
                Some(map) => map,
                None => return false,
            };
            let correspondants = map.correspondant_signer_list();

            for correspondant in correspondants.iter() {
                let vse_key_ = match self.vse_key(signer.to_owned(), correspondant.to_owned()) {
                    Some(key) => key,
                    None => return false,
                };
                let vse_key__ = match self.vse_key(correspondant.to_owned(), signer.to_owned()) {
                    Some(key) => key,
                    None => return false,
                };
                if vse_key_ != vse_key__ {
                    return false;
                }
            }
        }

        true
    }

    pub fn vse_key(&self, signer_1: [u8; 32], signer_2: [u8; 32]) -> Option<[u8; 32]> {
        for map in self.vse_keys.iter() {
            if map.signer_key() == signer_1 {
                if let Some(key) = map.vse_key(signer_2) {
                    return Some(key);
                }
            }
        }

        None
    }
}
