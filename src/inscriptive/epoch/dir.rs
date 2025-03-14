use super::epoch::Epoch;
use crate::{Network, EPOCH_DIRECTORY};
use secp::Point;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/// Directory for the operator quorum epoches.
pub struct EpochDirectory {
    // In-memory list.
    epochs: HashMap<u64, Epoch>,
    // In-storage db.
    db: sled::Db,
}

impl EpochDirectory {
    pub fn new(network: Network) -> Option<EPOCH_DIRECTORY> {
        let path = format!("{}/{}/{}", "db", network.to_string(), "node/dir/epoch");
        let db = sled::open(path).ok()?;

        let mut epochs = HashMap::<u64, Epoch>::new();

        for lookup in db.iter() {
            if let Ok((key, val)) = lookup {
                let height: u64 = u64::from_be_bytes(key.as_ref().try_into().ok()?);
                let epoch: Epoch = serde_json::from_slice(&val).ok()?;

                epochs.insert(height, epoch);
            }
        }

        let epoch_dir = EpochDirectory { epochs, db };

        Some(Arc::new(Mutex::new(epoch_dir)))
    }

    pub fn insert_epoch(&mut self, epoch: &Epoch) -> bool {
        let height = epoch.height();

        // Insert in-memory.
        if let Some(_) = self.epochs.insert(height, epoch.to_owned()) {
            return false;
        }

        // Insert in-db.
        match self.db.insert(height.to_be_bytes(), epoch.serialize()) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub fn group_keys(&self) -> Vec<Point> {
        self.epochs
            .iter()
            .map(|(_, epoch)| epoch.group_key())
            .collect()
    }
}
