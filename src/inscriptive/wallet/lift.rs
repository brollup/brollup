use crate::{txo::lift::Lift, Network, LIFT_WALLET};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Wallet for storing bare Lift outputs.
pub struct LiftWallet {
    height: u64,
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

        let height = match db.get([0x00]) {
            Ok(val) => match val {
                Some(val) => u64::from_be_bytes(val.as_ref().try_into().ok()?),
                None => 0,
            },
            Err(_) => return None,
        };

        for lookup in db.iter() {
            if let Ok((key, val)) = lookup {
                // Skip the key allocated for 'height' value.
                if key == [0x00] {
                    continue;
                }

                let lift: Lift = serde_json::from_slice(&val).ok()?;
                set.push(lift);
            }
        }

        let wallet = LiftWallet { height, set, db };

        Some(Arc::new(Mutex::new(wallet)))
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn set_height(&mut self, height: u64) {
        self.height = height;
        let _ = self.db.insert([0x00], height.to_be_bytes().to_vec());
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
