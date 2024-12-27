use std::collections::HashMap;

type SignerKey = [u8; 32];
type VSEKey = [u8; 32];

#[derive(Clone, PartialEq)]
pub struct KeyMap {
    signer: SignerKey,
    map: HashMap<SignerKey, VSEKey>,
}

impl KeyMap {
    pub fn new(signer: SignerKey) -> KeyMap {
        KeyMap {
            signer,
            map: HashMap::<SignerKey, VSEKey>::new(),
        }
    }

    pub fn signer_key(&self) -> SignerKey {
        self.signer
    }

    pub fn insert(&mut self, signer_key: SignerKey, vse_key: VSEKey) {
        self.map.insert(signer_key, vse_key);
    }

    pub fn correspondant_signer_list(&self) -> Vec<SignerKey> {
        let mut keys: Vec<SignerKey> = self.map.keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn full_signer_list(&self) -> Vec<SignerKey> {
        let mut full_list = Vec::<SignerKey>::new();

        full_list.push(self.signer_key());
        full_list.extend(self.correspondant_signer_list());
        full_list.sort();

        full_list
    }

    pub fn is_complete(&self, expected_signer_list_: Vec<SignerKey>) -> bool {
        let mut expected_signer_list = expected_signer_list_;
        expected_signer_list.sort();

        let self_signer_list = self.full_signer_list();

        if self_signer_list.len() == expected_signer_list.len() {
            for (index, self_signer) in self_signer_list.iter().enumerate() {
                if self_signer.to_owned() != expected_signer_list[index] {
                    return false;
                }
            }
            return true;
        }

        false
    }

    pub fn vse_key(&self, correspondant: SignerKey) -> Option<VSEKey> {
        Some(self.map.get(&correspondant)?.to_owned())
    }
}

pub struct Directory {
    signers: Vec<SignerKey>,
    vse_keys: Vec<KeyMap>,
}

impl Directory {
    pub fn new(signers: Vec<SignerKey>) -> Directory {
        Directory {
            signers,
            vse_keys: Vec::<KeyMap>::new(),
        }
    }

    pub fn signers(&self) -> Vec<SignerKey> {
        self.signers.clone()
    }

    pub fn insert(&mut self, map: KeyMap) -> bool {
        if self.signers.contains(&map.signer_key()) {
            if map.is_complete(self.signers()) {
                if !self.vse_keys.contains(&map) {
                    self.vse_keys.push(map);
                    return true;
                }
            }
        }
        false
    }

    pub fn map(&self, signer: SignerKey) -> Option<KeyMap> {
        for map in self.vse_keys.iter() {
            if map.signer_key() == signer {
                return Some(map.to_owned());
            }
        }

        None
    }

    pub fn is_complete(&self) -> bool {
        if self.vse_keys.len() != self.signers.len() {
            return false;
        }

        for map in self.vse_keys.iter() {
            if !map.is_complete(self.signers()) {
                return false;
            }
        }

        true
    }

    pub fn validate(&self) -> bool {
        if !self.is_complete() {
            return false;
        }

        for signer in self.signers.iter() {
            let map = match self.map(signer.to_owned()) {
                Some(map) => map,
                None => return false,
            };
            let correspondants = map.correspondant_signer_list();

            for correspondant in correspondants.iter() {
                let vse_key_ = match self.vse_key(signer.to_owned(), correspondant.to_owned()) {
                    Some(key) => key,
                    None => return false,
                };
                let vse_key__ = match self.vse_key(correspondant.to_owned(), signer.to_owned()) {
                    Some(key) => key,
                    None => return false,
                };
                if vse_key_ != vse_key__ {
                    return false;
                }
            }
        }

        true
    }

    pub fn vse_key(&self, signer_1: SignerKey, signer_2: SignerKey) -> Option<VSEKey> {
        for map in self.vse_keys.iter() {
            if map.signer_key() == signer_1 {
                if let Some(key) = map.vse_key(signer_2) {
                    return Some(key);
                }
            }
        }

        None
    }
}
