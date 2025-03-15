use super::epoch::Epoch;
use crate::{baked, into::IntoPoint, Network, EPOCH_DIRECTORY};
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
        let path = format!("{}/{}/{}", "db", network.to_string(), "dir/epoch");
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

    pub fn current_epoch_height(&self) -> u64 {
        match self.epochs.iter().max_by_key(|(&k, _)| k) {
            Some((&height, _)) => height,
            None => 0,
        }
    }

    pub fn next_epoch_height(&self) -> u64 {
        self.current_epoch_height() + 1
    }

    pub fn current_epoch(&self) -> Option<Epoch> {
        match self.epochs.iter().max_by_key(|(&k, _)| k) {
            Some((_, epoch)) => Some(epoch.to_owned()),
            None => None,
        }
    }

    pub fn operator_set(&self, network: Network) -> Vec<Point> {
        let mut operator_set = Vec::<Point>::new();

        // Fill with the initial operator set.
        {
            let initial_operator_set = match network {
                Network::Signet => baked::INITIAL_SIGNET_OPERATOR_SET,
                Network::Mainnet => baked::INITIAL_MAINNET_OPERATOR_SET,
            };

            operator_set.extend(
                initial_operator_set
                    .into_iter()
                    .filter_map(|op| op.into_point().ok()),
            );
        }

        // Fill with the epoch operator set.
        {
            operator_set.extend(
                self.epochs
                    .values()
                    .filter(|epoch| epoch.active())
                    .flat_map(|epoch| epoch.operator_keys()),
            );
        }

        operator_set
    }
}
