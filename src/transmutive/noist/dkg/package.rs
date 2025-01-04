use crate::schnorr::Bytes32;

use super::sharemap::DKGShareMap;

#[derive(Clone)]
pub struct DKGPackage {
    signer: [u8; 32],
    hiding: DKGShareMap,
    binding: DKGShareMap,
}

impl DKGPackage {
    pub fn new(secret_key: [u8; 32], signatories: Vec<[u8; 32]>) -> Option<Self> {
        let self_public = secret_key.secret_to_public()?;

        let hiding = DKGShareMap::new(secret_key, &signatories)?;
        let binding = DKGShareMap::new(secret_key, &signatories)?;

        Some(DKGPackage {
            signer: self_public,
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
}
