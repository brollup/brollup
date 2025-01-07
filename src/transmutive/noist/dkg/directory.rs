use super::session::DKGSession;
use crate::{noist::setup::setup::VSESetup, schnorr::challenge};
use secp::{MaybeScalar, Point, Scalar};
use std::collections::HashMap;

#[derive(Clone)]
pub struct DKGDirectory {
    sessions_db: sled::Db, // Database connection.
    vse_setup: VSESetup,
    batch_no: u64,                              // In-memory batch number.
    signatories: Vec<Point>,                    // Signatories list.
    sessions: HashMap<u64, (DKGSession, bool)>, // In-memory DKG sessions (nonce, (session, is_toxic)).
    nonce_bound: u64,
}
// 'db/signatory/dkg/batches/BATCH_NO' key:SESSION_NONCE
impl DKGDirectory {
    async fn new(batch_no: u64, signatories: &Vec<Point>, vse_setup: &VSESetup) -> Option<Self> {
        let mut signatories = signatories.clone();
        signatories.sort();

        // sessions path 'db/signatory/dkg/batches/BATCH_NO/sessions' key is SESSION_INDEX
        // manager path 'db/signatory/dkg/batches/manager' key is BATCH_NO
        let sessions_path = format!("{}/{}/sessions", "db/signatory/dkg/batches", batch_no);
        let sessions_db = sled::open(sessions_path).ok()?;

        let mut sessions = HashMap::<u64, (DKGSession, bool)>::new();

        // Insert DKG sessions to the memory.
        for lookup in sessions_db.iter() {
            if let Ok((nonce, session)) = lookup {
                let nonce: u64 = u64::from_be_bytes(nonce.as_ref().try_into().ok()?);
                let session: DKGSession = bincode::deserialize(session.as_ref()).ok()?;
                sessions.insert(nonce, (session, false));
            }
        }

        let nonce_bound = sessions.keys().max().unwrap_or(&0).to_owned();

        Some(DKGDirectory {
            sessions_db,
            vse_setup: vse_setup.to_owned(),
            batch_no,
            signatories,
            sessions,
            nonce_bound,
        })
    }

    pub fn sessions_db(&self) -> sled::Db {
        self.sessions_db.clone()
    }

    pub fn vse_setup(&self) -> &VSESetup {
        &self.vse_setup
    }

    pub fn batch_no(&self) -> u64 {
        self.batch_no
    }

    pub fn sessions(&self) -> HashMap<u64, (DKGSession, bool)> {
        self.sessions.clone()
    }

    pub fn nonce_bound(&self) -> u64 {
        self.nonce_bound
    }

    pub fn signatories(&self) -> Vec<Point> {
        self.signatories.clone()
    }

    pub async fn insert(&mut self, session: &DKGSession) -> bool {
        // Nonce session insertions are not allowed, if key session is not set.
        if let None = self.key_session().await {
            if session.nonce() != 0 {
                return false;
            }
        }

        if !session.verify(&self.vse_setup) {
            return false;
        }

        let session_nonce = session.nonce();

        if session_nonce <= self.nonce_bound {
            return false;
        }

        if let Ok(_) = self
            .sessions_db
            .insert(session.nonce().to_be_bytes(), session.serialize())
        {
            if let None = self
                .sessions
                .insert(session_nonce, (session.to_owned(), false))
            {
                self.nonce_bound = session_nonce;
                return true;
            }
        }
        false
    }

    pub async fn key_session(&self) -> Option<DKGSession> {
        Some(self.sessions.get(&0)?.0.to_owned())
    }

    pub async fn nonce_session(&self, nonce: u64) -> Option<(DKGSession, bool)> {
        // Nonce '0' is allocated for the group key.
        if nonce == 0 {
            return None;
        }
        Some(self.sessions.get(&nonce)?.to_owned())
    }

    pub async fn group_key(&self) -> Option<Point> {
        let group_key_session = self.key_session().await?;
        let group_key = group_key_session.group_combined_full_point(None, None)?;
        Some(group_key)
    }

    pub async fn group_nonce(&self, nonce: u64, message: [u8; 32]) -> Option<Point> {
        let nonce_session = self.nonce_session(nonce).await?.0;
        let group_key_bytes = self.group_key().await?.serialize_xonly();
        let group_nonce =
            nonce_session.group_combined_full_point(Some(group_key_bytes), Some(message))?;
        Some(group_nonce)
    }

    pub async fn challenge(&self, nonce: u64, message: [u8; 32]) -> Option<Scalar> {
        let group_nonce = self.group_nonce(nonce, message).await?;
        let group_key = self.group_key().await?;

        let challenge = challenge(
            group_nonce,
            group_key,
            message,
            crate::schnorr::SigningMode::BIP340,
        );

        match challenge {
            MaybeScalar::Valid(scalar) => return Some(scalar),
            MaybeScalar::Zero => return None,
        }
    }

    pub async fn set_toxic(&mut self, nonce: u64) -> bool {
        // Nonce '0' is allocated for the group key.
        if nonce == 0 {
            return false;
        }

        let (session, _) = match self.nonce_session(nonce).await {
            Some(session) => session,
            None => return false,
        };

        self.sessions.insert(nonce, (session, true));

        true
    }

    pub async fn remove_in_memory(&mut self, nonce: u64) -> bool {
        // Group key session cannot be removed.
        if nonce == 0 {
            return false;
        }

        match self.sessions.remove(&nonce) {
            Some(_) => return true,
            None => return false,
        };
    }

    pub async fn remove_db(&mut self, nonce: u64) -> bool {
        // Group key session cannot be removed.
        if nonce == 0 {
            return false;
        }

        match self.sessions_db.remove(nonce.to_be_bytes()) {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }

    pub fn num_nonce_sessions(&self) -> u64 {
        (self.sessions.len() as u64) - 1 // Total number of sessions minus the key session.
    }

    pub async fn new_session(&mut self) -> Option<DKGSession> {
        let new_nonce_bound = {
            match self.key_session().await {
                None => 0,
                Some(_) => {
                    let new_nonce_bound = self.nonce_bound + 1;
                    self.nonce_bound = new_nonce_bound;
                    new_nonce_bound
                }
            }
        };

        DKGSession::new(new_nonce_bound, &self.signatories)
    }

    pub async fn pick_session(&mut self) -> Option<DKGSession> {
        let session = self
            .sessions
            .iter()
            .filter(|(&key, &(_, is_toxic))| key != 0 && !is_toxic) // Exclude nonce: 0
            .min_by_key(|(&key, _)| key)
            .map(|(_, (session, _))| session)?
            .to_owned();

        let session_nonce = session.nonce();
        // A picked DKG session becomes toxic, and should be removed from db.
        self.set_toxic(session_nonce).await;
        self.remove_db(session_nonce).await;

        Some(session.to_owned())
    }
}
