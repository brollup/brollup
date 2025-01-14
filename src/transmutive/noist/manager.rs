use tokio::sync::Mutex;

use crate::DKG_DIRECTORY;

use super::{dkg::directory::DKGDirectory, setup::setup::VSESetup};
use std::{collections::HashMap, sync::Arc};

pub struct NOISTManager {
    directories: HashMap<u64, DKG_DIRECTORY>, // u64 setup no
    setup_db: sled::Db,
}

impl NOISTManager {
    pub fn new() -> Option<NOISTManager> {
        let setup_db = sled::open("db/noist/setup").ok()?;

        let mut directories = HashMap::<u64, DKG_DIRECTORY>::new();

        for lookup in setup_db.iter() {
            if let Ok((_, setup_)) = lookup {
                let setup: VSESetup = serde_json::from_slice(&setup_).ok()?;
                let setup_no = setup.setup_no();
                let dkg_directory = DKGDirectory::new(&setup)?;
                directories.insert(setup_no, Arc::new(Mutex::new(dkg_directory)));
            }
        }

        Some(NOISTManager {
            directories,
            setup_db,
        })
    }

    pub fn directories(&self) -> HashMap<u64, DKG_DIRECTORY> {
        self.directories.clone()
    }

    pub fn directory(&self, setup_no: u64) -> Option<DKG_DIRECTORY> {
        Some(Arc::clone(self.directories.get(&setup_no)?))
    }

    pub fn insert_setup(&mut self, setup: &VSESetup) -> bool {
        let setup_no = setup.setup_no();

        if self.directories.contains_key(&setup_no) {
            return false;
        };

        if let Err(_) = self
            .setup_db
            .insert(setup.setup_no().to_be_bytes(), setup.serialize())
        {
            return false;
        }

        let new_directory = match DKGDirectory::new(setup) {
            Some(directory) => directory,
            None => return false,
        };

        if let Some(_) = self
            .directories
            .insert(setup_no, Arc::new(Mutex::new(new_directory)))
        {
            return false;
        }

        true
    }
}

// db/noist/setup/ key: BATCH_NO -> VSESetup
// db/noist/batch/BATCH_NO/   key: "index_height" -> u64
// db/noist/batch/BATCH_NO/session/   key: SESSION_INDEX -> DKGSession
