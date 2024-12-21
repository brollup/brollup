use sha2::{Digest, Sha256};

pub trait Hash {
    fn hash(&self) -> [u8; 32];
}

impl Hash for Vec<u8> {
    fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self);
        let result = hasher.finalize();
        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&result);
        hash_array
    }
}

impl Hash for [u8; 32] {
    fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self);
        let result = hasher.finalize();
        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&result);
        hash_array
    }
}

impl Hash for [u8] {
    fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self);
        let result = hasher.finalize();
        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&result);
        hash_array
    }
}
