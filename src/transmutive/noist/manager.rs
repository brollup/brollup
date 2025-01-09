use std::collections::HashMap;

use super::{dkg::directory::DKGDirectory, setup::setup::VSESetup};

pub struct NOISTManager {
    directories: HashMap<u64, DKGDirectory>, // u64 setup no
    setup_db: sled::Db,
}

impl NOISTManager {
    pub fn new() -> Option<NOISTManager> {
        let setup_db = sled::open("db/noist/setup").ok()?;

        let mut directories = HashMap::<u64, DKGDirectory>::new();

        for lookup in setup_db.iter() {
            if let Ok((_, setup_)) = lookup {
                let setup: VSESetup = serde_json::from_slice(&setup_).ok()?;
                let setup_no = setup.no();
                let dkg_directory = DKGDirectory::new(&setup)?;
                directories.insert(setup_no, dkg_directory);
            }
        }

        Some(NOISTManager {
            directories,
            setup_db,
        })
    }

    pub fn directory(&self, setup_no: u64) -> Option<DKGDirectory> {
        Some(self.directories.get(&setup_no)?.to_owned())
    }

    pub fn new_setup(&mut self, setup: &VSESetup) -> bool {
        let setup_no = setup.no();

        if self.directories.contains_key(&setup_no) {
            return false;
        };

        if let Err(_) = self
            .setup_db
            .insert(setup.no().to_be_bytes(), setup.serialize())
        {
            return false;
        }

        let new_directory = match DKGDirectory::new(setup) {
            Some(directory) => directory,
            None => return false,
        };

        if let Some(_) = self.directories.insert(setup_no, new_directory) {
            return false;
        }

        true
    }
}

// db/noist/setup/ key: BATCH_NO -> VSESetup
// db/noist/batch/BATCH_NO/   key: "index_height" -> u64
// db/noist/batch/BATCH_NO/session/   key: SESSION_INDEX -> DKGSession
