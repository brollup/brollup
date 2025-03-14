use crate::{
    hash::{Hash, HashTag},
    schnorr::Sighash,
    txo::lift::Lift,
    valtype::account::Account,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Liftup {
    lift_prevtxos: Vec<Lift>,
}

impl Liftup {
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

    pub fn lifts(&self) -> Vec<Lift> {
        self.lift_prevtxos.clone()
    }

    pub fn len(&self) -> usize {
        self.lift_prevtxos.len()
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

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

impl Sighash for Liftup {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        for prevtxo in self.lift_prevtxos.iter() {
            let bytes = match prevtxo.outpoint() {
                Some(outpoint) => outpoint.bytes(),
                None => return [0; 32],
            };

            preimage.extend(bytes);
        }

        preimage.hash(Some(HashTag::SighashCombinator))
    }
}
