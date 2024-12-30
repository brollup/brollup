use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{vse, VSEDirectory};

const VSE_DIRECTORY_PATH: &str = "db/signatory/vse_directory";

pub struct Database {
    vse_directory_conn: sled::Db,
}

impl Database {
    pub fn new() -> Option<Self> {
        let vse_directory_conn = sled::open(VSE_DIRECTORY_PATH).ok()?;

        let database = Database { vse_directory_conn };

        Some(database)
    }

    pub fn vse_directory(&self) -> Option<VSEDirectory> {
        match self.vse_directory_conn.get(VSE_DIRECTORY_PATH).ok()? {
            Some(directory) => {
                let vse_directory: vse::Directory = bincode::deserialize(&directory).ok()?;
                return Some(Arc::new(Mutex::new(vse_directory)));
            }
            None => return None,
        }
    }

    pub async fn save_vse_directory(&self, vse_directory: &VSEDirectory) -> bool {
        let serialized = {
            let _vse_directory = vse_directory.lock().await;
            _vse_directory.serialize()
        };
        match self
            .vse_directory_conn
            .insert(VSE_DIRECTORY_PATH, serialized)
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
