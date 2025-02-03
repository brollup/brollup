use sha2::{Digest, Sha256};

use crate::baked;

#[derive(Copy, Clone, PartialEq)]
pub enum HashTag {
    VSEEncryptionAuth,
    SighashAuthenticable,
    SignatureChallenge,
    BIP340Challenge,
    GroupCommitment,
    BindingFactor,
    SharedSecret,
    SecretNonce,
    SecretKey,
    TapLeaf,
    TapBranch,
    TapTweak,
    TapSighash,
    // MuSig
    KeyAggList,
    KeyAggCoef,
    MusigNonceCoef,
}

impl HashTag {
    pub fn as_str(&self) -> String {
        match self {
            HashTag::VSEEncryptionAuth => format!("{}/{}", baked::PROJECT_TAG, "vseencryptionauth"),
            HashTag::SighashAuthenticable => format!("{}/{}", baked::PROJECT_TAG, "sighashauth"),
            HashTag::SignatureChallenge => format!("{}/{}", baked::PROJECT_TAG, "challenge"),
            HashTag::BIP340Challenge => format!("{}/{}", "BIP0340", "challenge"),
            HashTag::GroupCommitment => format!("{}/{}", baked::PROJECT_TAG, "groupcommitment"),
            HashTag::BindingFactor => format!("{}/{}", baked::PROJECT_TAG, "bindingfactor"),
            HashTag::SharedSecret => format!("{}/{}", baked::PROJECT_TAG, "sharedsecret"),
            HashTag::SecretNonce => format!("{}/{}", baked::PROJECT_TAG, "secretnonce"),
            HashTag::SecretKey => format!("{}/{}", baked::PROJECT_TAG, "secretkey"),
            HashTag::TapLeaf => format!("TapLeaf"),
            HashTag::TapBranch => format!("TapBranch"),
            HashTag::TapTweak => format!("TapTweak"),
            HashTag::TapSighash => format!("TapSighash"),
            HashTag::KeyAggList => format!("KeyAgg list"),
            HashTag::KeyAggCoef => format!("KeyAgg coefficient"),
            HashTag::MusigNonceCoef => format!("MuSig/noncecoef"),
        }
    }
}

pub fn sha256(preimage: &[u8]) -> [u8; 32] {
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
