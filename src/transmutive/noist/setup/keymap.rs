use crate::transmutive::{
    hash::{Hash, HashTag},
    noist::core::vse::encrypting_key_public,
    secp::{
        authenticable::AuthSighash,
        into::{FromSigTuple, IntoSigTuple},
        schnorr::{self, SchnorrSigningMode},
    },
};
use secp::{Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type SigTuple = (Point, Scalar);
type Proof = Option<Vec<u8>>;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VSEKeyMap {
    signatory: Point,
    map: HashMap<Point, (Point, SigTuple, Proof)>,
}

impl VSEKeyMap {
    pub fn new(secret_key: Scalar, signatories: &Vec<Point>) -> Option<VSEKeyMap> {
        let mut signatories = signatories.clone();
        signatories.sort();

        let public_key = secret_key.base_point_mul();

        let mut map = HashMap::<Point, (Point, SigTuple, Proof)>::new();

        for signatory in signatories {
            let vse_public = encrypting_key_public(secret_key, signatory.to_owned());

            let message = {
                let mut preimage = Vec::<u8>::with_capacity(97);
                preimage.extend(public_key.serialize_xonly()); // 32-byte well-known key.
                preimage.extend(signatory.serialize_xonly()); // 32-byte well-known key.
                preimage.extend(vse_public.serialize()); // 33-byte encryption key.
                preimage.hash(Some(HashTag::VSEEncryptionAuth))
            };

            let auth_sig =
                schnorr::sign(secret_key.serialize(), message, SchnorrSigningMode::Brollup)?;
            let auth_sig_tuple = auth_sig.into_sig_tuple()?;

            map.insert(signatory.to_owned(), (vse_public, auth_sig_tuple, None));
        }

        Some(VSEKeyMap {
            signatory: public_key,
            map,
        })
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

    pub fn signatory(&self) -> Point {
        self.signatory.clone()
    }

    pub fn map(&self) -> HashMap<Point, (Point, SigTuple, Proof)> {
        self.map.clone()
    }

    pub fn signatories(&self) -> Vec<Point> {
        let mut keys: Vec<Point> = self.map.keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn verify(&self, signatories: &Vec<Point>) -> bool {
        let mut signatories = signatories.clone();
        signatories.sort();

        if self.signatories() != signatories.to_owned() {
            return false;
        }

        for (signatory, (encryption_key, sig_tuple, _proof)) in self.map.iter() {
            let signature = sig_tuple.from_sig_tuple();

            let message = {
                let mut preimage = Vec::<u8>::with_capacity(97);
                preimage.extend(self.signatory.serialize_xonly()); // 32-byte well-known key.
                preimage.extend(signatory.serialize_xonly()); // 32-byte well-known key.
                preimage.extend(encryption_key.serialize()); // 33-byte encryption key.
                preimage.hash(Some(HashTag::VSEEncryptionAuth))
            };

            if !schnorr::verify(
                self.signatory.serialize_xonly(),
                message,
                signature,
                SchnorrSigningMode::Brollup,
            ) {
                return false;
            }

            // TODO: _proof
        }
        true
    }

    pub fn remove_signatory(&mut self, signatory: &Point) -> bool {
        println!(
            "keymap remove_signatory: {}",
            hex::encode(signatory.serialize_xonly())
        );
        match self.map.remove(signatory) {
            Some(_) => return true,
            None => return false,
        }
    }

    pub fn vse_point(&self, correspondant: Point) -> Option<Point> {
        Some(self.map.get(&correspondant)?.0)
    }

    pub fn vse_key(&self, correspondant: Point) -> Option<[u8; 32]> {
        Some(self.vse_point(correspondant)?.serialize_xonly())
    }

    pub fn ordered_map(&self) -> Vec<(Point, (Point, SigTuple, Proof))> {
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

        for map in self.map.iter() {
            println!(
                "  {} -> {}",
                hex::encode(map.0.serialize_xonly()),
                hex::encode(map.1 .0.serialize_xonly())
            );
        }
        println!("");
    }
}

impl AuthSighash for VSEKeyMap {
    fn auth_sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();
        preimage.extend(self.signatory.serialize_xonly());

        for (correspondant, (vse_point, sig_tuple, proof)) in self.ordered_map().iter() {
            preimage.extend(correspondant.serialize_xonly());
            preimage.extend(vse_point.serialize_xonly());
            preimage.extend(sig_tuple.0.serialize_xonly());
            preimage.extend(sig_tuple.1.serialize());
            match proof {
                Some(proof) => {
                    preimage.push(0x01);
                    preimage.extend(proof)
                }
                None => preimage.push(0x00),
            }
        }

        preimage.hash(Some(HashTag::Sighash))
    }
}
