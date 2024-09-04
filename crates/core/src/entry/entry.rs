use super::transfer::Transfer;
use crate::{
    encoding::{cpe::CompactPayloadEncoding, serialize::Serialize, sighash::Sighash},
    hash::{tagged_hash, HashTag},
    secp::schnorr::{sign_schnorr, verify_schnorr, verify_schnorr_batch, SecpError, SignFlag},
};
use bit_vec::BitVec;

pub enum Entry {
    Transfer(Transfer),
}

impl Entry {
    pub fn msg_sender(&self) -> [u8; 32] {
        let msg_sender_key = match self {
            Entry::Transfer(transfer) => transfer.msg_sender().key(),
        };
        msg_sender_key.serialize()
    }
}

impl CompactPayloadEncoding for Entry {
    fn to_cpe(&self) -> BitVec {
        match self {
            Entry::Transfer(transfer) => transfer.to_cpe(),
        }
    }
}

impl Sighash for Entry {
    fn sighash(&self, prev_state_hash: [u8; 32]) -> [u8; 32] {
        let mut sighash_preimage = Vec::<u8>::new();

        sighash_preimage.extend(prev_state_hash);

        let (serialized_entry, sighash_tag) = match self {
            Entry::Transfer(transfer) => (transfer.serialize(), HashTag::SighashTransfer),
        };

        sighash_preimage.extend(serialized_entry);

        tagged_hash(sighash_preimage, sighash_tag)
    }
}
pub trait EntrySignature {
    fn sign(&self, secret_key: [u8; 32], prev_state_hash: [u8; 32]) -> Result<[u8; 64], SecpError>;
    fn verify(&self, signature: [u8; 64], prev_state_hash: [u8; 32]) -> Result<(), SecpError>;
}

pub trait EntryBatchVerification {
    fn batch_verify(&self, signature: [u8; 65], prev_state_hash: [u8; 32])
        -> Result<(), SecpError>;
}

impl EntrySignature for Entry {
    fn sign(&self, secret_key: [u8; 32], prev_state_hash: [u8; 32]) -> Result<[u8; 64], SecpError> {
        // Message is the sighash of the entry.
        let message = self.sighash(prev_state_hash);

        // Sign the message with the entry signing method.
        sign_schnorr(secret_key, message, SignFlag::EntrySign)
    }

    fn verify(&self, signature: [u8; 64], prev_state_hash: [u8; 32]) -> Result<(), SecpError> {
        // Message is the sighash of the entry.
        let message = self.sighash(prev_state_hash);

        let public_key = self.msg_sender();

        verify_schnorr(public_key, message, signature, SignFlag::EntrySign)
    }
}

impl EntryBatchVerification for Vec<Entry> {
    fn batch_verify(
        &self,
        signatures_sum: [u8; 65],
        prev_state_hash: [u8; 32],
    ) -> Result<(), SecpError> {
        let mut messages = Vec::<[u8; 32]>::with_capacity(self.len());
        let mut public_keys = Vec::<[u8; 32]>::with_capacity(self.len());

        for entry in self {
            messages.push(entry.sighash(prev_state_hash));
            public_keys.push(entry.msg_sender());
        }

        verify_schnorr_batch(signatures_sum, public_keys, messages, SignFlag::EntrySign)
    }
}
