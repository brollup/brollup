use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{db, SIGNATORY_DB};

use super::setup::VSESetup;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VSEDirectory {
    setups: HashMap<u64, VSESetup>,
}

impl VSEDirectory {
    pub async fn new(db: &SIGNATORY_DB) -> Option<Self> {
        let _db = db.lock().await;

        let directory = match _db.vse_directory_conn().get(db::VSE_DIRECTORY_PATH).ok()? {
            Some(data) => bincode::deserialize(&data).ok()?,
            None => VSEDirectory {
                setups: HashMap::<u64, VSESetup>::new(),
            },
        };

        Some(directory)
    }

    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        match bincode::deserialize(&bytes) {
            Ok(directory) => Some(directory),
            Err(_) => None,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match bincode::serialize(&self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn no_reserved(&self, no: u64) -> bool {
        match self.setups.get(&no) {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn setups(&self) -> HashMap<u64, VSESetup> {
        self.setups.clone()
    }

    pub async fn insert(&mut self, setup: &VSESetup, db: &SIGNATORY_DB) -> bool {
        if !setup.validate() {
            return false;
        };

        match self.setups.insert(setup.no(), setup.clone()) {
            Some(_) => return false,
            None => {
                self.prune();
                self.save(db).await
            }
        }
    }

    pub async fn save(&self, db: &SIGNATORY_DB) -> bool {
        let _db = db.lock().await;

        match _db
            .vse_directory_conn()
            .insert(db::VSE_DIRECTORY_PATH, self.serialize())
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub fn prune(&mut self) {
        if self.setups.len() > 3 {
            if let Some(&min_key) = self.setups.keys().min() {
                self.setups.remove(&min_key);
            }
        }
    }

    pub fn setup(&self, no: u64) -> Option<VSESetup> {
        Some(self.setups.get(&no)?.clone())
    }

    pub async fn print(&self) {
        for (batch_no, setup) in self.setups().await.iter() {
            println!("Setup #{} :", batch_no);
            setup.print();
            println!("");
        }
    }
}
