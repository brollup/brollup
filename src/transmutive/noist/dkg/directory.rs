use super::session::DKGSession;
use crate::schnorr::challenge;
use secp::{MaybeScalar, Point, Scalar};
use std::collections::HashMap;

#[derive(Clone)]
pub struct DKGDirectory {
    db: sled::Db,                       // Database connection.
    batch_no: u64,                      // In-memory batch number.
    sessions: HashMap<u64, DKGSession>, // In-memory DKG sessions (nonce, session).
}
// 'db/signatory/dkg/BATCH_NO' key:SESSION_NONCE
impl DKGDirectory {
    async fn new(batch_no: u64) -> Option<Self> {
        let db_path = format!("{}/{}", "db/signatory/dkg", batch_no);
        let db = sled::open(db_path).ok()?;

        let mut sessions = HashMap::<u64, DKGSession>::new();

        // Insert DKG sessions to the memory.
        for lookup in db.iter() {
            if let Ok((nonce, session)) = lookup {
                let nonce: u64 = u64::from_be_bytes(nonce.as_ref().try_into().ok()?);
                let session: DKGSession = bincode::deserialize(session.as_ref()).ok()?;
                sessions.insert(nonce, session);
            }
        }

        Some(DKGDirectory {
            db,
            batch_no,
            sessions,
        })
    }

    pub fn db(&self) -> sled::Db {
        self.db.clone()
    }

    pub fn batch_no(&self) -> u64 {
        self.batch_no
    }

    pub fn sessions(&self) -> HashMap<u64, DKGSession> {
        self.sessions.clone()
    }

    async fn insert(&mut self, session: &DKGSession) -> bool {
        if let Ok(_) = self
            .db
            .insert(session.nonce().to_be_bytes(), session.serialize())
        {
            if let None = self.sessions.insert(session.nonce(), session.to_owned()) {
                return true;
            }
        }
        false
    }

    async fn group_key_session(&self) -> Option<DKGSession> {
        Some(self.sessions.get(&0)?.to_owned())
    }

    async fn group_nonce_session(&self, nonce: u64) -> Option<DKGSession> {
        // Nonce '0' is allocated for the group key.
        if nonce == 0 {
            return None;
        }
        Some(self.sessions.get(&nonce)?.to_owned())
    }

    async fn group_key(&self) -> Option<Point> {
        let group_key_session = self.group_key_session().await?;
        let group_key = group_key_session.group_combined_full_point(None, None)?;
        Some(group_key)
    }

    async fn group_nonce(&self, nonce: u64, message: [u8; 32]) -> Option<Point> {
        let nonce_session = self.group_nonce_session(nonce).await?;
        let group_key_bytes = self.group_key().await?.serialize_xonly();
        let group_nonce =
            nonce_session.group_combined_full_point(Some(group_key_bytes), Some(message))?;
        Some(group_nonce)
    }

    async fn challenge(&self, nonce: u64, message: [u8; 32]) -> Option<Scalar> {
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
}
