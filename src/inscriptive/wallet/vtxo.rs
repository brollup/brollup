use crate::{txo::vtxo::VTXO, Network, VTXO_WALLET};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Wallet for storing virtual utxos.
pub struct VTXOWallet {
    // In-memory list.
    set: Vec<VTXO>,
    // In-storage db.
    db: sled::Db,
}

impl VTXOWallet {
    pub fn new(network: Network) -> Option<VTXO_WALLET> {
        let path = format!("{}/{}/{}", "db", network.to_string(), "wallet/vtxo");
        let db = sled::open(path).ok()?;

        let mut set = Vec::<VTXO>::new();

        for lookup in db.iter() {
            if let Ok((_, val)) = lookup {
                let vtxo: VTXO = serde_json::from_slice(&val).ok()?;
                set.push(vtxo);
            }
        }

        let wallet = VTXOWallet { set, db };

        Some(Arc::new(Mutex::new(wallet)))
    }

    pub fn set(&self) -> Vec<VTXO> {
        self.set.clone()
    }

    /// Inserts a new Lift into wallet.
    pub fn insert(&mut self, vtxo: &VTXO) -> bool {
        let outpoint = match vtxo.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is an overlap in the set.

        if self
            .set
            .iter()
            .any(|vtxo| vtxo.outpoint() == Some(outpoint))
        {
            return false;
        }

        // Insert in-memory.
        self.set.push(vtxo.to_owned());

        // Insert in-db.
        match self
            .db
            .insert(&outpoint.bytes(), vtxo.to_owned().serialize())
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    /// Removes a Lift from the wallet.
    pub fn remove(&mut self, vtxo: &VTXO) -> bool {
        let outpoint = match vtxo.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is one in the set.
        let Some(index) = self
            .set
            .iter()
            .position(|vtxo| vtxo.outpoint() == Some(outpoint))
        else {
            return false;
        };

        // Remove in-memory.
        self.set.remove(index);

        // Remove in-db.
        match self.db.remove(&outpoint.bytes()) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
