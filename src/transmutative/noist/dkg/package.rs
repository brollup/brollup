use crate::{
    transmutative::hash::{Hash, HashTag},
    transmutative::noist::setup::setup::VSESetup,
    transmutative::secp::authenticable::AuthSighash,
};
use secp::{Point, Scalar};
use serde::{Deserialize, Serialize};

use super::sharemap::DKGShareMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct DKGPackage {
    signatory: Point,
    hiding: DKGShareMap,
    binding: DKGShareMap,
}

impl DKGPackage {
    pub fn new(secret_key: Scalar, signatories: &Vec<Point>) -> Option<Self> {
        let public_key = secret_key.base_point_mul();

        let hiding = DKGShareMap::new(secret_key, public_key, &signatories)?;
        let binding = DKGShareMap::new(secret_key, public_key, &signatories)?;

        let package = DKGPackage {
            signatory: public_key,
            hiding,
            binding,
        };

        Some(package)
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

    pub fn hiding(&self) -> DKGShareMap {
        self.hiding.clone()
    }

    pub fn binding(&self) -> DKGShareMap {
        self.binding.clone()
    }

    pub fn is_complete(&self, signatories: &Vec<Point>) -> bool {
        if !self.hiding.is_complete(signatories) {
            return false;
        }

        if !self.binding.is_complete(signatories) {
            return false;
        }

        true
    }

    pub fn vss_verify(&self) -> bool {
        if !self.hiding.vss_verify() {
            return false;
        }

        if !self.binding.vss_verify() {
            return false;
        }

        true
    }

    pub fn vse_verify(&self, setup: &VSESetup) -> bool {
        if !self.hiding.vse_verify(setup) {
            return false;
        }

        if !self.binding.vse_verify(setup) {
            return false;
        }

        true
    }

    pub fn print(&self) {
        println!(
            "Package by {} :",
            hex::encode(self.signatory.serialize_xonly())
        );
        println!("Hiding Sharemap :");
        self.hiding.print();

        println!("\n Binding Sharemap :");
        self.binding.print();
    }
}

impl AuthSighash for DKGPackage {
    fn auth_sighash(&self) -> [u8; 32] {
        let mut preimage = Vec::<u8>::new();
        preimage.extend(self.signatory.serialize_xonly());
        preimage.extend(self.hiding.auth_sighash());
        preimage.extend(self.binding.auth_sighash());
        preimage.hash(Some(HashTag::Sighash))
    }
}
