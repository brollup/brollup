use super::{dkg::directory::DKGDirectory, session::SessionCtx, setup::setup::VSESetup};
use crate::{musig::session::MusigSessionCtx, DKG_DIRECTORY, DKG_MANAGER};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct DKGManager {
    directories: HashMap<u64, DKG_DIRECTORY>,
    setup_db: sled::Db,
}

impl DKGManager {
    pub fn new() -> Option<DKG_MANAGER> {
        let setup_db = sled::open("db/noist/setup").ok()?;

        let mut directories = HashMap::<u64, DKG_DIRECTORY>::new();

        for lookup in setup_db.iter() {
            if let Ok((_, setup_)) = lookup {
                let setup: VSESetup = serde_json::from_slice(&setup_).ok()?;
                let setup_height = setup.height();
                let dkg_directory: DKGDirectory = DKGDirectory::new(&setup)?;
                directories.insert(setup_height, Arc::new(Mutex::new(dkg_directory)));
            }
        }

        let manager_ = Some(DKGManager {
            directories,
            setup_db,
        })?;

        Some(Arc::new(Mutex::new(manager_)))
    }

    pub fn directories(&self) -> HashMap<u64, DKG_DIRECTORY> {
        self.directories.clone()
    }

    pub fn directory(&self, dir_height: u64) -> Option<DKG_DIRECTORY> {
        Some(Arc::clone(self.directories.get(&dir_height)?))
    }

    pub fn insert_setup(&mut self, setup: &VSESetup) -> bool {
        let setup_height = setup.height();

        if self.directories.contains_key(&setup_height) {
            return false;
        };

        if let Err(_) = self
            .setup_db
            .insert(setup.height().to_be_bytes(), setup.serialize())
        {
            return false;
        }

        let new_directory = match DKGDirectory::new(setup) {
            Some(directory) => directory,
            None => return false,
        };

        if let Some(_) = self
            .directories
            .insert(setup_height, Arc::new(Mutex::new(new_directory)))
        {
            return false;
        }

        true
    }

    pub fn setup_height(&self) -> u64 {
        self.directories.keys().max().unwrap_or(&0).to_owned()
    }

    pub async fn pick_signing_session(
        &self,
        dir_height: u64,
        message: [u8; 32],
        musig_ctx: Option<MusigSessionCtx>,
        toxic: bool,
    ) -> Option<SessionCtx> {
        let dkg_dir: DKG_DIRECTORY = self.directory(dir_height)?;
        let mut dkg_dir_ = dkg_dir.lock().await;
        let nonce_height = dkg_dir_.pick_index()?;
        dkg_dir_.signing_session(message, nonce_height, musig_ctx, toxic)
    }

    pub async fn signing_session(
        &self,
        dir_height: u64,
        message: [u8; 32],
        nonce_height: u64,
        musig_ctx: Option<MusigSessionCtx>,
        toxic: bool,
    ) -> Option<SessionCtx> {
        let dkg_dir: DKG_DIRECTORY = self.directory(dir_height)?;
        let mut dkg_dir_ = dkg_dir.lock().await;
        let session = dkg_dir_.signing_session(message, nonce_height, musig_ctx, toxic)?;

        Some(session)
    }
}
