use std::collections::HashMap;

use secp::Point;
use serde::{Deserialize, Serialize};

use crate::{into::IntoPoint, schnorr::Authenticable};

use super::keymap::VSEKeyMap;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VSESetup {
    no: u64,
    signatories: Vec<Point>,
    maps: HashMap<Point, Authenticable<VSEKeyMap>>,
}

impl VSESetup {
    pub fn new(signatories: &Vec<[u8; 32]>, no: u64) -> Option<Self> {
        let signatories = {
            let mut list = Vec::<Point>::new();
            for signatory in signatories {
                let signatory_point = signatory.into_point().ok()?;
                list.push(signatory_point);
            }
            list
        };

        let vse_setup = VSESetup {
            no,
            signatories,
            maps: HashMap::<Point, Authenticable<VSEKeyMap>>::new(),
        };

        Some(vse_setup)
    }

    pub fn no(&self) -> u64 {
        self.no
    }

    pub fn signatories(&self) -> Vec<Point> {
        self.signatories.clone()
    }

    pub fn maps(&self) -> HashMap<Point, Authenticable<VSEKeyMap>> {
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
        if self.signatories.contains(&map.object().signatory()) {
            let map_key_point = match map.key().into_point() {
                Ok(point) => point,
                Err(_) => return false,
            };

            if let None = self.maps.get(&map_key_point) {
                if map.object().is_complete(&self.signatories()) {
                    let map_key_point = match map.key().into_point() {
                        Ok(point) => point,
                        Err(_) => return false,
                    };
                    self.maps.insert(map_key_point, map);
                }
                return true;
            }
        }
        false
    }

    pub fn auth_map(&self, correspondant: [u8; 32]) -> Option<Authenticable<VSEKeyMap>> {
        let correspondant_point = correspondant.into_point().ok()?;
        let map = self.maps().get(&correspondant_point)?.clone();
        Some(map)
    }

    pub fn map(&self, correspondant: [u8; 32]) -> Option<VSEKeyMap> {
        Some(self.auth_map(correspondant)?.object())
    }

    pub fn is_complete(&self) -> bool {
        if self.maps.len() != self.signatories.len() {
            return false;
        }

        for (_, map) in self.maps.iter() {
            if !map.object().is_complete(&self.signatories()) {
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
                if !self.signatories().contains(key) {
                    return false;
                }

                let map_key_point = match map.key().into_point() {
                    Ok(point) => point,
                    Err(_) => return false,
                };

                if key != &map_key_point {
                    return false;
                }
                if !map.authenticate() {
                    return false;
                }
            }

            // 2. Sig matching.
            {
                let correspondants = map.object().correspondants();

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

    pub fn vse_point(&self, signer_1: Point, signer_2: Point) -> Option<Point> {
        for (key, map) in self.maps.iter() {
            if key == &signer_1 {
                if let Some(point) = map.object().vse_point(signer_2) {
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
        if self.maps.len() == 0 {
            println!("None.");
        }

        for (key, map) in self.maps().iter() {
            println!("{}", hex::encode(key.serialize_xonly()));
            for triple in map.object().map().iter() {
                let proof = {
                    match triple.1 .1.clone() {
                        Some(proof) => hex::encode(proof),
                        None => "None".to_owned(),
                    }
                };
                println!(
                    "    {} -> vse_key: {} proof: {}",
                    hex::encode(triple.0.serialize_xonly()),
                    hex::encode(triple.1 .0.serialize_xonly()),
                    proof
                );
            }
            println!("");
        }
    }
}
