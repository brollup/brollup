use crate::{txo::lift::Lift, Network};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type LIFT_WALLET = Arc<Mutex<LiftWallet>>;

/// Wallet for storing bare Lift outputs.
pub struct LiftWallet {
    // In-memory Lift set.
    lifts: Vec<Lift>,
    // In-storage Lifts db.
    lifts_db: sled::Db,
}

impl LiftWallet {
    pub fn new(network: Network) -> Option<LIFT_WALLET> {
        // Collect lifts from db.

        let lifts_path = format!("{}/{}/{}", "db", network.to_string(), "wallet/lift");
        let lifts_db = sled::open(lifts_path).ok()?;

        let mut lift_set = Vec::<Lift>::new();

        for lookup in lifts_db.iter() {
            if let Ok((_, val)) = lookup {
                if let Ok(lift) = serde_json::from_slice::<Lift>(&val) {
                    lift_set.push(lift);
                }
            }
        }

        // Construct lift wallet.
        let lift_wallet = LiftWallet {
            lifts: lift_set,
            lifts_db,
        };

        Some(Arc::new(Mutex::new(lift_wallet)))
    }

    pub fn lifts(&self) -> Vec<Lift> {
        self.lifts.clone()
    }

    /// Inserts a new Lift into the wallet.
    pub fn insert_lift(&mut self, lift: &Lift) -> bool {
        let outpoint = match lift.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is an overlap in the set.
        if self.lifts.iter().any(|l| l.outpoint() == Some(outpoint)) {
            return false;
        }

        // Insert in-memory.
        self.lifts.push(lift.to_owned());

        // Insert in-db.
        match self
            .lifts_db
            .insert(&outpoint.bytes(), lift.to_owned().serialize())
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Removes a Lift from the wallet.
    pub fn remove_lift(&mut self, lift: &Lift) -> bool {
        let outpoint = match lift.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is one in the set.
        let Some(index) = self
            .lifts
            .iter()
            .position(|l| l.outpoint() == Some(outpoint))
        else {
            return false;
        };

        // Remove in-memory.
        self.lifts.remove(index);

        // Remove in-db.
        match self.lifts_db.remove(&outpoint.bytes()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
