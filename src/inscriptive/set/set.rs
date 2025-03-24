use super::{
    utxo_set::{UTXOSet, UTXO_SET},
    vtxo_set::{VTXOSet, VTXO_SET},
};
use crate::Network;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Guarded set for storing coins.
pub type COIN_SET = Arc<Mutex<CoinSet>>;

/// Set for storing bare and virtual coins (UTXOs and VTXOs).
pub struct CoinSet {
    utxo_set: UTXO_SET,
    vtxo_set: VTXO_SET,
}

impl CoinSet {
    /// Creates the CoinSet instance.
    pub fn new(network: Network) -> Option<COIN_SET> {
        // Construct utxo set.
        let utxo_set = UTXOSet::new(network)?;

        // Construct vtxo set.
        let vtxo_set = VTXOSet::new(network)?;

        let set = CoinSet { utxo_set, vtxo_set };

        Some(Arc::new(Mutex::new(set)))
    }

    /// Returns the UTXO set.
    pub fn utxo_set(&self) -> UTXO_SET {
        Arc::clone(&self.utxo_set)
    }

    /// Returns the VTXO set.
    pub fn vtxo_set(&self) -> VTXO_SET {
        Arc::clone(&self.vtxo_set)
    }
}
