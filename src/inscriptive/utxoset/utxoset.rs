use crate::{
    txn::ext::{OutpointExt, TxOutExt},
    Network,
};
use bitcoin::{OutPoint, TxOut};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub type UTXO_SET = Arc<Mutex<UTXOSet>>;

/// Lookup for storing bare UTXOs.
pub struct UTXOSet {
    // In-memory utxo set.
    utxos: HashMap<OutPoint, TxOut>,
    // In-storage utxo set db.
    utxos_db: sled::Db,
}

impl UTXOSet {
    /// Creates a new UTXOSet.
    pub fn new(network: Network) -> Option<UTXO_SET> {
        // Collect utxos from db.
        let utxos_path = format!("{}/{}/{}", "db", network.to_string(), "utxo_set");
        let utxos_db = sled::open(utxos_path).ok()?;

        let mut utxos = HashMap::<OutPoint, TxOut>::new();

        // Load utxos from db.
        for lookup in utxos_db.iter() {
            if let Ok((key, val)) = lookup {
                // Deserialize outpoint.
                let outpoint_bytes: [u8; 36] = key.as_ref().try_into().ok()?;
                let outpoint = OutPoint::from_bytes36(&outpoint_bytes)?;

                // Deserialize txout.
                let txout = TxOut::from_bytes(val.as_ref())?;

                // Insert utxo.
                utxos.insert(outpoint, txout);
            }
        }

        // Construct utxoset.
        let utxoset = UTXOSet { utxos, utxos_db };

        Some(Arc::new(Mutex::new(utxoset)))
    }

    /// Returns the number of utxos in the set.
    pub fn num_utxos(&self) -> usize {
        self.utxos.len()
    }

    /// Returns the utxo at the given script pubkey.
    pub fn txout_by_outpoint(&self, outpoint: &OutPoint) -> Option<TxOut> {
        self.utxos.get(outpoint).cloned()
    }

    /// Inserts a txout into the set.
    pub fn insert_txout(&mut self, outpoint: &OutPoint, txout: &TxOut) {
        if let None = self.utxos.insert(outpoint.clone(), txout.clone()) {
            let _ = self.utxos_db.insert(&outpoint.bytes_36(), txout.bytes());
        }
    }

    /// Removes a txout from the set.
    pub fn remove_txout(&mut self, outpoint: &OutPoint) {
        if let Some(_) = self.utxos.remove(outpoint) {
            let _ = self.utxos_db.remove(&outpoint.bytes_36());
        }
    }
}
