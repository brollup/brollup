use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    hash::{Hash, HashTag},
    into::{IntoPoint, IntoScalar},
    noist::vse::encrypting_key_public,
    schnorr::{Bytes32, Sighash},
};

type CorrespondantKey = [u8; 32];
type CorrespondantVSEKey = [u8; 32];
type VSEProof = Option<Vec<u8>>;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VSEKeyMap {
    key: [u8; 32],
    map: HashMap<CorrespondantKey, (CorrespondantVSEKey, VSEProof)>,
}

impl VSEKeyMap {
    pub fn new(self_secret: [u8; 32], list: &Vec<[u8; 32]>) -> Option<VSEKeyMap> {
        let self_public = self_secret.secret_to_public()?;

        let mut map = HashMap::<CorrespondantKey, (CorrespondantVSEKey, VSEProof)>::new();

        for to_public in list {
            if to_public != &self_public {
                let correspondant_vse_key = encrypting_key_public(
                    self_secret.into_scalar().ok()?,
                    to_public.into_point().ok()?,
                );

                map.insert(*to_public, (correspondant_vse_key.serialize_xonly(), None));
            }
        }

        Some(VSEKeyMap {
            key: self_public,
            map,
        })
    }

    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        match bincode::deserialize(&bytes) {
            Ok(keymap) => Some(keymap),
            Err(_) => None,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match bincode::serialize(&self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn map(&self) -> HashMap<CorrespondantKey, (CorrespondantVSEKey, VSEProof)> {
        self.map.clone()
    }

    pub fn key(&self) -> [u8; 32] {
        self.key
    }

    pub fn map_list(&self) -> Vec<[u8; 32]> {
        let mut keys: Vec<[u8; 32]> = self.map.keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn full_list(&self) -> Vec<[u8; 32]> {
        let mut full_list = Vec::<[u8; 32]>::new();

        full_list.push(self.key());
        full_list.extend(self.map_list());
        full_list.sort();

        full_list
    }

    pub fn is_complete(&self, expected_list: &Vec<[u8; 32]>) -> bool {
        let expected_list = {
            let mut expected_list_ = expected_list.clone();
            expected_list_.sort();
            expected_list_
        };

        let full_list = self.full_list();

        if full_list.len() == expected_list.len() {
            for (index, key) in full_list.iter().enumerate() {
                if key != &expected_list[index] {
                    return false;
                }
            }
            return true;
        }

        false
    }

    pub fn vse_key(&self, correspondant: [u8; 32]) -> Option<[u8; 32]> {
        Some(self.map.get(&correspondant)?.0.to_owned())
    }
}

impl Sighash for VSEKeyMap {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        preimage.extend(self.key());

        let mut maps: Vec<(CorrespondantKey, (CorrespondantVSEKey, VSEProof))> =
            self.map().into_iter().collect();
        maps.sort_by(|a, b| a.0.cmp(&b.0));

        for (signer_key, (vse_key, proof)) in maps.iter() {
            preimage.extend(signer_key);
            preimage.extend(vse_key);
            match proof {
                Some(proof) => {
                    preimage.push(0x01);
                    preimage.extend(proof)
                }
                None => preimage.push(0x00),
            }
        }

        preimage.hash(Some(HashTag::SighashAuthenticable))
    }
}
