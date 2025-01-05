use crate::{noist::setup::setup::VSESetup, schnorr::Bytes32};

use super::sharemap::DKGShareMap;

#[derive(Clone)]
pub struct DKGPackage {
    signer: [u8; 32],
    hiding: DKGShareMap,
    binding: DKGShareMap,
}

impl DKGPackage {
    pub fn new(secret_key: [u8; 32], signatories: &Vec<[u8; 32]>) -> Option<Self> {
        let public_key = secret_key.secret_to_public()?;

        let hiding = DKGShareMap::new(secret_key, public_key, &signatories)?;
        let binding = DKGShareMap::new(secret_key, public_key, &signatories)?;

        Some(DKGPackage {
            signer: public_key,
            hiding,
            binding,
        })
    }

    pub fn signer(&self) -> [u8; 32] {
        self.signer.clone()
    }

    pub fn hiding(&self) -> DKGShareMap {
        self.hiding.clone()
    }

    pub fn binding(&self) -> DKGShareMap {
        self.binding.clone()
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
