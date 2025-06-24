use crate::{
    constructive::{
        entity::account::account::Account, entry::combinator::combinator_type::CombinatorType,
        txo::lift::Lift,
    },
    transmutative::{
        hash::{Hash, HashTag},
        secp::authenticable::AuthSighash,
    },
};
use bitcoin::hashes::Hash as _;
use serde::{Deserialize, Serialize};

/// A `Liftup` is a collection of `Lift`s that are being lifted up.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Liftup {
    pub lift_prevtxos: Vec<Lift>,
}

impl Liftup {
    /// Creates a new `Liftup` from a list of `Lift`s.  
    pub fn new(lifts: Vec<Lift>) -> Option<Liftup> {
        let mut lift_prevtxos = Vec::<Lift>::new();

        for lift in lifts.iter() {
            match lift.outpoint() {
                Some(_) => lift_prevtxos.push(lift.to_owned()),
                None => return None,
            }
        }

        let liftup = Liftup { lift_prevtxos };

        Some(liftup)
    }

    /// Returns the list of `Lift`s in the `Liftup`.
    pub fn lifts(&self) -> Vec<Lift> {
        self.lift_prevtxos.clone()
    }

    /// Returns the number of `Lift`s in the `Liftup`.
    pub fn num_lifts(&self) -> usize {
        self.lift_prevtxos.len()
    }

    /// Serializes the `Liftup` to a byte vector.
    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    /// Validates the `Liftup` against an `Account`.
    pub fn validate_account(&self, account: Account) -> bool {
        for lift in self.lift_prevtxos.iter() {
            if let None = lift.outpoint() {
                return false;
            }

            if lift.account_key() != account.key() {
                return false;
            }
        }

        true
    }
}

impl AuthSighash for Liftup {
    fn auth_sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        for prevtxo in self.lift_prevtxos.iter() {
            match prevtxo.outpoint() {
                Some(outpoint) => {
                    preimage.extend(outpoint.txid.to_byte_array());
                    preimage.extend(outpoint.vout.to_le_bytes());
                }
                None => return [0; 32],
            }
        }

        preimage.hash(Some(HashTag::SighashCombinator(CombinatorType::Liftup)))
    }
}
