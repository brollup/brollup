pub const VSE_DIRECTORY_PATH: &str = "db/signatory/vse_directory";

pub struct Signatory {
    vse_directory_conn: sled::Db,
}

impl Signatory {
    pub fn new() -> Option<Self> {
        let vse_directory_conn = sled::open(VSE_DIRECTORY_PATH).ok()?;

        let database = Signatory { vse_directory_conn };

        Some(database)
    }
    pub fn vse_directory_conn(&self) -> &sled::Db {
        &self.vse_directory_conn
    }
}
