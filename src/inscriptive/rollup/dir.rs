use crate::{rpc::bitcoin_rpc::get_chain_height, rpcholder::RPCHolder, Network, ROLLUP_DIRECTORY};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Directory for the rollup state.
pub struct RollupDirectory {
    synced: bool,
    // Bitcoin sync height.
    bitcoin_sync_height: u64,
    // Rollup sync height.
    rollup_sync_height: u64,
    // In-storage db.
    db: sled::Db,
}

impl RollupDirectory {
    pub fn new(network: Network) -> Option<ROLLUP_DIRECTORY> {
        let path = format!("{}/{}/{}", "db", network.to_string(), "dir/rollup");
        let db = sled::open(path).ok()?;

        let bitcoin_sync_height: u64 = db
            .get(b"bitcoin_sync_height")
            .ok()
            .flatten()
            .and_then(|val| val.as_ref().try_into().ok().map(u64::from_be_bytes))
            .unwrap_or(0);

        let rollup_sync_height: u64 = db
            .get(b"rollup_sync_height")
            .ok()
            .flatten()
            .and_then(|val| val.as_ref().try_into().ok().map(u64::from_be_bytes))
            .unwrap_or(0);

        let rollup_dir = RollupDirectory {
            synced: false,
            bitcoin_sync_height,
            rollup_sync_height,
            db,
        };

        Some(Arc::new(Mutex::new(rollup_dir)))
    }

    pub fn set_synced(&mut self, synced: bool) {
        self.synced = synced;
    }

    pub fn is_synced(&self) -> bool {
        self.synced
    }

    /// Returns the bitcoin sync height.
    pub fn bitcoin_sync_height(&self) -> u64 {
        self.bitcoin_sync_height
    }

    /// Returns the rollup sync height.
    pub fn rollup_sync_height(&self) -> u64 {
        self.rollup_sync_height
    }

    /// Sets the bitcoin sync height.
    pub fn set_bitcoin_sync_height(&mut self, height: u64) {
        // Update in-memory.
        self.bitcoin_sync_height = height;

        // Update in-db.
        let _ = self
            .db
            .insert(b"bitcoin_sync_height", height.to_be_bytes().to_vec());
    }

    /// Sets the rollup sync height.
    pub fn set_rollup_sync_height(&mut self, height: u64) {
        // Update in-memory.
        self.rollup_sync_height = height;

        // Update in-db.
        let _ = self
            .db
            .insert(b"rollup_sync_height", height.to_be_bytes().to_vec());
    }

    /// Returns whether the rollup is fully synced.
    pub fn is_fully_synced(&self, rpc_holder: &RPCHolder) -> bool {
        let bitcoin_sync_height = match get_chain_height(rpc_holder) {
            Ok(height) => height,
            Err(_) => return false,
        };

        self.bitcoin_sync_height >= bitcoin_sync_height
    }
}
