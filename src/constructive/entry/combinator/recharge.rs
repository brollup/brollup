use crate::{
    hash::{Hash, HashTag},
    schnorr::Sighash,
    txo::vtxo::VTXO,
    valtype::account::Account,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Recharge {
    recharge_vtxos: Vec<VTXO>,
}

impl Recharge {
    pub fn new(vtxos: Vec<VTXO>) -> Option<Recharge> {
        let mut recharge_vtxos = Vec::<VTXO>::new();

        for vtxo in vtxos.iter() {
            match vtxo.outpoint() {
                Some(_) => recharge_vtxos.push(vtxo.to_owned()),
                None => return None,
            }
        }

        let liftup = Recharge { recharge_vtxos };

        Some(liftup)
    }

    pub fn vtxos(&self) -> Vec<VTXO> {
        self.recharge_vtxos.clone()
    }

    pub fn len(&self) -> usize {
        self.recharge_vtxos.len()
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn validate_account(&self, account: Account) -> bool {
        for vtxo in self.recharge_vtxos.iter() {
            if let None = vtxo.outpoint() {
                return false;
            }

            if vtxo.account_key() != account.key() {
                return false;
            }
        }

        true
    }
}

impl Sighash for Recharge {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        for vtxo in self.recharge_vtxos.iter() {
            let bytes = match vtxo.outpoint() {
                Some(outpoint) => outpoint.bytes(),
                None => return [0; 32],
            };

            preimage.extend(bytes);
        }

        preimage.hash(Some(HashTag::SighashCombinator))
    }
}
