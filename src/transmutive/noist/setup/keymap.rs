use crate::{
    hash::{Hash, HashTag},
    into::{IntoPoint, IntoScalar},
    noist::vse::encrypting_key_public,
    schnorr::{Bytes32, Sighash},
};
use secp::Point;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VSEKeyMap {
    signatory: Point,
    map: HashMap<Point, (Point, Option<Vec<u8>>)>, // Point (correspondant key) -> Point (vse key)
}

impl VSEKeyMap {
    pub fn new(self_secret: [u8; 32], list: &Vec<Point>) -> Option<VSEKeyMap> {
        let self_point = self_secret.secret_to_public()?.into_point().ok()?;

        let mut map = HashMap::<Point, (Point, Option<Vec<u8>>)>::new();

        for to_public in list {
            let to_vse_point =
                encrypting_key_public(self_secret.into_scalar().ok()?, to_public.to_owned());

            map.insert(to_public.to_owned(), (to_vse_point, None));
        }

        Some(VSEKeyMap {
            signatory: self_point,
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

    pub fn map(&self) -> HashMap<Point, (Point, Option<Vec<u8>>)> {
        self.map.clone()
    }

    pub fn signatory(&self) -> Point {
        self.signatory
    }

    pub fn signatories(&self) -> Vec<Point> {
        let mut keys: Vec<Point> = self.map.keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn is_complete(&self, expected_list: &Vec<Point>) -> bool {
        let signatories = self.signatories();

        let mut expected_list = expected_list.clone();
        expected_list.sort();

        if signatories.len() == expected_list.len() {
            for (index, key) in signatories.iter().enumerate() {
                if key != &expected_list[index] {
                    return false;
                }
            }

            return true;
        }

        false
    }

    pub fn vse_point(&self, correspondant: Point) -> Option<Point> {
        Some(self.map.get(&correspondant)?.0)
    }

    pub fn vse_key(&self, correspondant: Point) -> Option<[u8; 32]> {
        Some(self.vse_point(correspondant)?.serialize_xonly())
    }

    pub fn ordered_map(&self) -> Vec<(Point, (Point, Option<Vec<u8>>))> {
        let mut sorted_map: Vec<_> = self.map.iter().collect();
        sorted_map.sort_by(|a, b| a.0.cmp(b.0));
        sorted_map
            .into_iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn print(&self) {
        println!(
            "Self key: {}",
            hex::encode(self.signatory.serialize_xonly())
        );

        for map in self.map() {
            println!(
                "  {} -> {}",
                hex::encode(map.0.serialize_xonly()),
                hex::encode(map.1 .0.serialize_xonly())
            );
        }
        println!("");
    }
}

impl Sighash for VSEKeyMap {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();
        preimage.extend(self.signatory().serialize_xonly());

        for (correspondant, (vse_point, proof)) in self.ordered_map().iter() {
            preimage.extend(correspondant.serialize_xonly());
            preimage.extend(vse_point.serialize_xonly());
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
