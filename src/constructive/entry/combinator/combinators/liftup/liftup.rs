use crate::constructive::txo::lift::Lift;
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
    pub fn validate_account(&self, account_key: [u8; 32]) -> bool {
        for lift in self.lift_prevtxos.iter() {
            if let None = lift.outpoint() {
                return false;
            }

            if lift.account_key().serialize_xonly() != account_key {
                return false;
            }
        }

        true
    }
}
