use crate::vse;

const VSE_DIRECTORY_PATH: &str = "vse_directory";

pub struct Database {
    vse_directory_conn: sled::Db,
}

impl Database {
    pub fn new() -> Option<Self> {
        let vse_directory_conn = sled::open(VSE_DIRECTORY_PATH).ok()?;

        let database = Database { vse_directory_conn };

        Some(database)
    }

    pub fn vse_directory(&self) -> Option<vse::Directory> {
        match self.vse_directory_conn.get(VSE_DIRECTORY_PATH).ok()? {
            Some(directory) => {
                let vse_directory: vse::Directory = bincode::deserialize(&directory).ok()?;
                return Some(vse_directory);
            }
            None => return None,
        }
    }

    pub fn save_vse_directory(&self, vse_directory: &vse::Directory) -> bool {
        let serialized = vse_directory.serialize();
        match self
            .vse_directory_conn
            .insert(VSE_DIRECTORY_PATH, serialized)
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
