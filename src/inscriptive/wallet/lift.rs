use crate::{txo::lift::Lift, Network, LIFT_WALLET};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Wallet for storing bare Lift outputs.
pub struct LiftWallet {
    bitcoin_sync_height: u64,
    // In-memory list.
    set: Vec<Lift>,
    // In-storage db.
    db: sled::Db,
}

impl LiftWallet {
    pub fn new(network: Network) -> Option<LIFT_WALLET> {
        let path = format!("{}/{}/{}", "db", network.to_string(), "wallet/lift");
        let db = sled::open(path).ok()?;

        let mut set = Vec::<Lift>::new();

        let bitcoin_sync_height: u64 = db
            .get(b"bitcoin_sync_height")
            .ok()
            .flatten()
            .and_then(|val| val.as_ref().try_into().ok().map(u64::from_be_bytes))
            .unwrap_or(0);

        for lookup in db.iter().flatten() {
            let (key, val) = lookup;

            // Skip the key allocated for 'bitcoin_sync_height' value.
            if key.as_ref() == b"bitcoin_sync_height" {
                continue;
            }

            if let Ok(lift) = serde_json::from_slice::<Lift>(&val) {
                set.push(lift);
            } else {
                eprintln!("Failed to deserialize Lift from DB");
            }
        }

        let wallet = LiftWallet {
            bitcoin_sync_height,
            set,
            db,
        };

        Some(Arc::new(Mutex::new(wallet)))
    }

    pub fn bitcoin_sync_height(&self) -> u64 {
        self.bitcoin_sync_height
    }

    pub fn set_bitcoin_sync_height(&mut self, height: u64) {
        self.bitcoin_sync_height = height;
        let _ = self
            .db
            .insert(b"bitcoin_sync_height", height.to_be_bytes().to_vec());
    }

    pub fn set(&self) -> Vec<Lift> {
        self.set.clone()
    }

    /// Inserts a new Lift into the wallet.
    pub fn insert(&mut self, lift: &Lift) -> bool {
        let outpoint = match lift.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is an overlap in the set.
        if self.set.iter().any(|l| l.outpoint() == Some(outpoint)) {
            return false;
        }

        // Insert in-memory.
        self.set.push(lift.to_owned());

        // Insert in-db.
        match self
            .db
            .insert(&outpoint.bytes(), lift.to_owned().serialize())
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Removes a Lift from the wallet.
    pub fn remove(&mut self, lift: &Lift) -> bool {
        let outpoint = match lift.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is one in the set.
        let Some(index) = self.set.iter().position(|l| l.outpoint() == Some(outpoint)) else {
            return false;
        };

        // Remove in-memory.
        self.set.remove(index);

        // Remove in-db.
        match self.db.remove(&outpoint.bytes()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
