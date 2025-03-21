use crate::{txo::vtxo::VTXO, Network};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type VTXO_WALLET = Arc<Mutex<VTXOWallet>>;

/// Wallet for storing VTXOs.
pub struct VTXOWallet {
    // In memory VTXO set.
    vtxos: Vec<VTXO>,
    // In-storage VTXO set.
    vtxos_db: sled::Db,
}

impl VTXOWallet {
    pub fn new(network: Network) -> Option<VTXO_WALLET> {
        // Collect VTXOs from db.

        let vtxos_path = format!("{}/{}/{}", "db", network.to_string(), "wallet/vtxo");
        let vtxos_db = sled::open(vtxos_path).ok()?;

        let mut vtxo_set = Vec::<VTXO>::new();

        for lookup in vtxos_db.iter() {
            if let Ok((_, val)) = lookup {
                let vtxo: VTXO = serde_json::from_slice(&val).ok()?;
                vtxo_set.push(vtxo);
            }
        }

        let wallet = VTXOWallet {
            vtxos: vtxo_set,
            vtxos_db,
        };

        Some(Arc::new(Mutex::new(wallet)))
    }

    pub fn vtxos(&self) -> Vec<VTXO> {
        self.vtxos.clone()
    }

    /// Inserts a new VTXO into wallet.
    pub fn insert_vtxo(&mut self, vtxo: &VTXO) -> bool {
        let outpoint = match vtxo.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is an overlap in the set.

        if self
            .vtxos
            .iter()
            .any(|vtxo| vtxo.outpoint() == Some(outpoint))
        {
            return false;
        }

        // Insert in-memory.
        self.vtxos.push(vtxo.to_owned());

        // Insert in-db.
        match self
            .vtxos_db
            .insert(&outpoint.bytes(), vtxo.to_owned().serialize())
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    /// Removes a VTXO from the wallet.
    pub fn remove_vtxo(&mut self, vtxo: &VTXO) -> bool {
        let outpoint = match vtxo.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is one in the set.
        let Some(index) = self
            .vtxos
            .iter()
            .position(|vtxo| vtxo.outpoint() == Some(outpoint))
        else {
            return false;
        };

        // Remove in-memory.
        self.vtxos.remove(index);

        // Remove in-db.
        match self.vtxos_db.remove(&outpoint.bytes()) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
