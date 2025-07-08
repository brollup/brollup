use crate::constructive::txo::vtxo::VTXO;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Recharge {
    pub recharge_vtxos: Vec<VTXO>,
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

    /// Validation from the broader Entry context.
    pub fn entry_validation(&self, account_key: [u8; 32]) -> bool {
        for vtxo in self.recharge_vtxos.iter() {
            if let None = vtxo.outpoint() {
                return false;
            }

            if vtxo.account_key().serialize_xonly() != account_key {
                return false;
            }
        }

        true
    }
}
