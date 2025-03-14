use crate::{txo::lift::Lift, Network, LIFT_WALLET};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Wallet for storing bare Lift utxos.
pub struct LiftWallet {
    // In-memory list.
    set: Vec<Lift>,
    // In-storage db.
    db: sled::Db,
}

impl LiftWallet {
    pub fn new(network: Network) -> Option<LIFT_WALLET> {
        let path = format!("{}/{}/{}", "db", network.to_string(), "node/wallet/lift");
        let db = sled::open(path).ok()?;

        let mut set = Vec::<Lift>::new();

        for lookup in db.iter() {
            if let Ok((_, val)) = lookup {
                let lift: Lift = serde_json::from_slice(&val).ok()?;

                set.push(lift);
            }
        }

        let wallet = LiftWallet { set, db };

        Some(Arc::new(Mutex::new(wallet)))
    }

    pub fn set(&self) -> Vec<Lift> {
        self.set.clone()
    }

    /// Inserts a new Lift into wallet.
    pub fn insert(&mut self, lift: &Lift) -> bool {
        let outpoint = match lift.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is an overlap in the set.

        if self
            .set
            .iter()
            .any(|lift| lift.outpoint() == Some(outpoint))
        {
            return false;
        }

        // Insert in-memory.
        self.set.push(lift.to_owned());

        // Insert in-db.
        match self
            .db
            .insert(outpoint.bytes(), lift.to_owned().serialize())
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    /// Removes a Lift from the wallet.
    pub fn remove(&mut self, lift: &Lift) -> bool {
        let outpoint = match lift.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is one in the set.
        let Some(index) = self
            .set
            .iter()
            .position(|lift| lift.outpoint() == Some(outpoint))
        else {
            return false;
        };

        // Remove in-memory.
        self.set.remove(index);

        // Remove in-db.
        match self.db.remove(outpoint.bytes()) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
