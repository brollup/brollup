#![allow(dead_code)]

use ripemd::Ripemd160;
use sha2::Digest as _;
use sha2::Sha256;
use sha2::Sha512;

type Bytes = Vec<u8>;

pub fn sha_256(data: impl AsRef<[u8]>) -> [u8; 32] {
    let hash: [u8; 32] = Sha256::new().chain_update(&data).finalize().into();
    hash
}

pub fn hash_256(data: impl AsRef<[u8]>) -> [u8; 32] {
    let hash: [u8; 32] = sha_256(sha_256(data));
    hash
}

pub fn sha_512(data: impl AsRef<[u8]>) -> [u8; 64] {
    let hash: [u8; 64] = Sha512::new().chain_update(&data).finalize().into();
    hash
}

pub fn ripemd_160(data: impl AsRef<[u8]>) -> [u8; 20] {
    let mut ripemd_160_hash = Ripemd160::new();
    ripemd_160_hash.update(data);

    let hash: [u8; 20] = ripemd_160_hash.finalize().into();
    hash
}

pub fn hash_160(data: impl AsRef<[u8]>) -> [u8; 20] {
    let hash: [u8; 20] = ripemd_160(sha_256(data));
    hash
}

pub enum HashTag {
    TapLeaf,
    TapBranch,
    TapTweak,
    SighashTransfer,
    SighashCall,
    SighashLiftup,
    SighashLiftdown,
    SighashRecharge,
    SighashReserved,
    DeterministicNonce,
    BIP0340Challenge,
    EntryChallenge,
    ProtocolMessageChallenge,
    CustomMessageChallenge,
    CustomTag(String),
}

pub fn tagged_hash(data: impl AsRef<[u8]>, tag: HashTag) -> [u8; 32] {
    let mut preimage = Vec::<u8>::new();

    let tag_digest = match tag {
        HashTag::TapLeaf => Sha256::digest("TapLeaf"),
        HashTag::TapBranch => Sha256::digest("TapBranch"),
        HashTag::TapTweak => Sha256::digest("TapTweak"),
        HashTag::SighashTransfer => Sha256::digest("Brollup/sighashtransfer"),
        HashTag::SighashCall => Sha256::digest("Brollup/sighashcall"),
        HashTag::SighashLiftup => Sha256::digest("Brollup/sighashliftup"),
        HashTag::SighashLiftdown => Sha256::digest("Brollup/sighashliftdown"),
        HashTag::SighashRecharge => Sha256::digest("Brollup/sighashrecharge"),
        HashTag::SighashReserved => Sha256::digest("Brollup/sighashreserved"),
        HashTag::DeterministicNonce => Sha256::digest("Brollup/deterministicnonce"),
        HashTag::BIP0340Challenge => Sha256::digest("BIP0340/challenge"),
        HashTag::EntryChallenge => Sha256::digest("Brollup/entry/challenge"),
        HashTag::ProtocolMessageChallenge => Sha256::digest("Brollup/protocolmessage/challenge"),
        HashTag::CustomMessageChallenge => Sha256::digest("Brollup/custommessage/challenge"),
        HashTag::CustomTag(tag) => Sha256::digest(tag),
    };

    preimage.extend(tag_digest);
    preimage.extend(tag_digest);
    preimage.extend(data.as_ref());

    sha_256(preimage)
}
