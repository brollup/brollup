use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::schnorr::Authenticable;

use super::keymap::VSEKeyMap;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VSESetup {
    no: u64,
    signers: Vec<[u8; 32]>,
    maps: HashMap<[u8; 32], Authenticable<VSEKeyMap>>,
}

impl VSESetup {
    pub fn new(signers: &Vec<[u8; 32]>, no: u64) -> VSESetup {
        VSESetup {
            no,
            signers: signers.clone(),
            maps: HashMap::<[u8; 32], Authenticable<VSEKeyMap>>::new(),
        }
    }

    pub fn no(&self) -> u64 {
        self.no
    }

    pub fn signers(&self) -> Vec<[u8; 32]> {
        self.signers.clone()
    }

    pub fn maps(&self) -> HashMap<[u8; 32], Authenticable<VSEKeyMap>> {
        self.maps.clone()
    }

    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        match bincode::deserialize(&bytes) {
            Ok(directory) => Some(directory),
            Err(_) => None,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match bincode::serialize(&self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn insert(&mut self, map: Authenticable<VSEKeyMap>) -> bool {
        if self.signers.contains(&map.object().key()) {
            if let None = self.maps.get(&map.key()) {
                if map.object().is_complete(&self.signers()) {
                    self.maps.insert(map.key(), map);
                }
                return true;
            }
        }
        false
    }

    pub fn auth_map(&self, signer: [u8; 32]) -> Option<Authenticable<VSEKeyMap>> {
        let map = self.maps().get(&signer)?.clone();
        Some(map)
    }

    pub fn map(&self, signer: [u8; 32]) -> Option<VSEKeyMap> {
        Some(self.auth_map(signer)?.object())
    }

    pub fn is_complete(&self) -> bool {
        if self.maps.len() != self.signers.len() {
            return false;
        }

        for (_, map) in self.maps.iter() {
            if !map.object().is_complete(&self.signers()) {
                return false;
            }
        }

        true
    }

    pub fn validate(&self) -> bool {
        // 0. Completeness
        if !self.is_complete() {
            return false;
        }

        for (key, map) in self.maps().iter() {
            // 1. Auth sigs
            {
                if !self.signers().contains(key) {
                    return false;
                }
                if key != &map.key() {
                    return false;
                }
                if !map.authenticate() {
                    return false;
                }
            }

            // 2. Sig matching.
            {
                let correspondants = map.object().map_list();

                for correspondant in correspondants.iter() {
                    let vse_key_ = match self.vse_key(key.to_owned(), correspondant.to_owned()) {
                        Some(key) => key,
                        None => return false,
                    };
                    let vse_key__ = match self.vse_key(correspondant.to_owned(), key.to_owned()) {
                        Some(key) => key,
                        None => return false,
                    };
                    if vse_key_ != vse_key__ {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn vse_key(&self, signer_1: [u8; 32], signer_2: [u8; 32]) -> Option<[u8; 32]> {
        for (key, map) in self.maps.iter() {
            if key == &signer_1 {
                if let Some(key) = map.object().vse_key(signer_2) {
                    return Some(key);
                }
            }
        }

        None
    }

    pub fn print(&self) {
        if self.maps.len() == 0 {
            println!("None.");
        }

        for (key, map) in self.maps().iter() {
            println!("{}", hex::encode(key));
            for triple in map.object().map().iter() {
                let proof = {
                    match triple.1 .1.clone() {
                        Some(proof) => hex::encode(proof),
                        None => "None".to_owned(),
                    }
                };
                println!(
                    "    {} -> vse_key: {} proof: {}",
                    hex::encode(triple.0),
                    hex::encode(triple.1 .0),
                    proof
                );
            }
            println!("");
        }
    }
}
