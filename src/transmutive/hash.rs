use sha2::{Digest, Sha256};

use crate::baked;

#[derive(Copy, Clone, PartialEq)]
pub enum HashTag {
    SighashAuthenticable,
    SignatureChallenge,
    GroupCommitment,
    BindingFactor,
    SharedSecret,
    SecretNonce,
    SecretKey,
}

impl HashTag {
    pub fn as_str(&self) -> String {
        let str = match self {
            HashTag::SighashAuthenticable => "sighash/authenticable",
            HashTag::SignatureChallenge => "challenge",
            HashTag::GroupCommitment => "groupcommitment",
            HashTag::BindingFactor => "bindingfactor",
            HashTag::SharedSecret => "sharedsecret",
            HashTag::SecretNonce => "secretnonce",
            HashTag::SecretKey => "secretkey",
        };
        format!("{}/{}", baked::PROJECT_TAG, str)
    }
}

fn sha256(preimage: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let result = hasher.finalize();
    let mut hash_array = [0u8; 32];
    hash_array.copy_from_slice(&result);
    hash_array
}

pub trait Hash {
    fn hash(&self, tag: Option<HashTag>) -> [u8; 32];
}

impl<T> Hash for T
where
    T: AsRef<[u8]>,
{
    fn hash(&self, tag: Option<HashTag>) -> [u8; 32] {
        let tag_hash = match tag {
            Some(tag) => sha256(tag.as_str().as_bytes()),
            None => [0xffu8; 32],
        };

        let mut preimage = Vec::<u8>::new();

        preimage.extend(tag_hash);
        preimage.extend(tag_hash);
        preimage.extend(self.as_ref());

        sha256(&preimage)
    }
}
