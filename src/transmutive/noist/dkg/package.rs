use secp::Point;
use serde::{Deserialize, Serialize};

use crate::{
    hash::Hash,
    into::IntoPoint,
    noist::setup::setup::VSESetup,
    schnorr::{Bytes32, Sighash},
};

use super::sharemap::DKGShareMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct DKGPackage {
    signatory: Point,
    hiding: DKGShareMap,
    binding: DKGShareMap,
}

impl DKGPackage {
    pub fn new(secret_key: [u8; 32], signatories: &Vec<[u8; 32]>) -> Option<Self> {
        let public_key = secret_key.secret_to_public()?;

        let hiding = DKGShareMap::new(secret_key, public_key, &signatories)?;
        let binding = DKGShareMap::new(secret_key, public_key, &signatories)?;

        let package = DKGPackage {
            signatory: public_key.into_point().ok()?,
            hiding,
            binding,
        };

        Some(package)
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

    pub fn is_complete(&self, signatories: &Vec<[u8; 32]>) -> bool {
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
        println!("Hiding Sharemap :");
        self.hiding.print();

        println!("\n Binding Sharemap :");
        self.binding.print();
    }
}

impl Sighash for DKGPackage {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage = Vec::<u8>::new();
        preimage.extend(self.signatory.serialize_xonly());
        preimage.extend(self.hiding.sighash());
        preimage.extend(self.binding.sighash());
        preimage.hash(Some(crate::hash::HashTag::SighashAuthenticable))
    }
}
