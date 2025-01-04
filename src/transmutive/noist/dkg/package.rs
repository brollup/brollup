use super::sharemap::DKGShareMap;

#[derive(Clone)]
pub struct DKGPackage {
    hiding: DKGShareMap,
    binding: DKGShareMap,
}

impl DKGPackage {
    pub fn new(secret_key: [u8; 32], signatories: Vec<[u8; 32]>) -> Option<Self> {
        let hiding = DKGShareMap::new(secret_key, &signatories)?;
        let binding = DKGShareMap::new(secret_key, &signatories)?;

        Some(DKGPackage { hiding, binding })
    }

    pub fn hiding(&self) -> DKGShareMap {
        self.hiding.clone()
    }

    pub fn binding(&self) -> DKGShareMap {
        self.binding.clone()
    }
}
