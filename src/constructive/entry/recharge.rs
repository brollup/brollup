use crate::txo::vtxo::VTXO;
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
}
