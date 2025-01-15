use crate::into::IntoPoint;

use super::keymap::VSEKeyMap;
use secp::Point;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VSESetup {
    height: u64,
    signatories: Vec<Point>,
    map: HashMap<Point, VSEKeyMap>,
}

impl VSESetup {
    pub fn new(signatories: &Vec<Point>, height: u64) -> Option<Self> {
        let mut signatories = signatories.clone();
        signatories.sort();

        let vse_setup = VSESetup {
            height,
            signatories,
            map: HashMap::<Point, VSEKeyMap>::new(),
        };

        Some(vse_setup)
    }

    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        match serde_json::from_slice(bytes) {
            Ok(keymap) => Some(keymap),
            Err(_) => None,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn signatories(&self) -> Vec<Point> {
        self.signatories.clone()
    }

    pub fn map(&self) -> HashMap<Point, VSEKeyMap> {
        self.map.clone()
    }

    pub fn insert_keymap(&mut self, keymap: VSEKeyMap) -> bool {
        if self.signatories.contains(&keymap.signatory()) {
            if let None = self.map.get(&keymap.signatory()) {
                if keymap.verify(&self.signatories) {
                    if let None = self.map.insert(keymap.signatory(), keymap) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn keymap(&self, correspondant: &Point) -> Option<VSEKeyMap> {
        let map = self.map.get(correspondant)?.clone();
        Some(map)
    }

    pub fn is_signatory(&self, key: [u8; 32]) -> bool {
        let signatory = match key.into_point() {
            Ok(point) => point,
            Err(_) => return false,
        };

        self.signatories.contains(&signatory)
    }

    fn remove_signatory(&mut self, signatory: &Point) -> bool {
        if !self.signatories.contains(&signatory) {
            return false;
        }

        self.signatories.retain(|&x| x != signatory.to_owned());

        for (_, map) in self.map.iter_mut() {
            if !map.remove_signatory(&signatory) {
                return false;
            }
        }

        true
    }

    pub fn remove_missing(&mut self) {
        let mut missing_signatories = Vec::<Point>::new();

        for signatory in self.signatories.iter() {
            if let None = self.map.get(signatory) {
                missing_signatories.push(signatory.to_owned());
            }
        }

        for missing_signatory in missing_signatories.iter() {
            self.remove_signatory(missing_signatory);
        }
    }

    pub fn verify(&self) -> bool {
        // 0. Completeness
        if self.map.len() != self.signatories.len() {
            return false;
        }

        for (_, map) in self.map.iter() {
            if !map.verify(&self.signatories) {
                return false;
            }
        }

        for (key, map) in self.map.iter() {
            // 1. Auth sigs
            if !self.signatories.contains(key) {
                return false;
            }

            // 2. Sig matching.
            {
                for signatory in map.signatories().iter() {
                    if signatory != key {
                        let vse_key_ = match self.vse_key(key.to_owned(), signatory.to_owned()) {
                            Some(key) => key,
                            None => return false,
                        };
                        let vse_key__ = match self.vse_key(signatory.to_owned(), key.to_owned()) {
                            Some(key) => key,
                            None => return false,
                        };

                        if vse_key_ != vse_key__ {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    pub fn vse_point(&self, signer_1: Point, signer_2: Point) -> Option<Point> {
        for (key, map) in self.map.iter() {
            if key == &signer_1 {
                if let Some(point) = map.vse_point(signer_2) {
                    return Some(point);
                }
            }
        }

        None
    }

    pub fn vse_key(&self, signer_1: Point, signer_2: Point) -> Option<[u8; 32]> {
        Some(self.vse_point(signer_1, signer_2)?.serialize_xonly())
    }

    pub fn print(&self) {
        if self.map.len() == 0 {
            println!("None.");
        }

        for (key, map) in self.map.iter() {
            println!("{}", hex::encode(key.serialize_xonly()));
            for (signatory, (vse_key, _, proof)) in map.map().iter() {
                let proof = {
                    match proof.to_owned() {
                        Some(proof) => hex::encode(proof),
                        None => "None".to_owned(),
                    }
                };
                println!(
                    "    {} -> vse_key: {} proof: {}",
                    hex::encode(signatory.serialize_xonly()),
                    hex::encode(vse_key.serialize_xonly()),
                    proof
                );
            }
            println!("");
        }
    }
}
