use super::session::DKGSession;
use crate::{
    noist::{
        lagrance::{interpolating_value, lagrance_index, lagrance_index_list},
        setup::setup::VSESetup,
    },
    schnorr::challenge,
};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};
use std::collections::HashMap;

#[derive(Clone)]
pub struct DKGDirectory {
    setup: VSESetup,                    // VSE setup.
    sessions: HashMap<u64, DKGSession>, // In-memory DKG sessions (index, session).
    sessions_db: sled::Db,              // Database connection.
    index_height: u64,
    index_height_db: sled::Db,
}
// 'db/signatory/dkg/batches/BATCH_NO' key:SESSION_NONCE
impl DKGDirectory {
    pub fn new(setup: &VSESetup) -> Option<Self> {
        let batch_no = setup.setup_no();
        // sessions path 'db/signatory/dkg/batches/BATCH_NO/sessions' key is SESSION_INDEX
        // manager path 'db/signatory/dkg/batches/manager' key is BATCH_NO
        let mut index_height: u64 = 0;

        let index_height_path = format!("{}/{}", "db/noist/dkgdir/", batch_no);
        let index_height_db = sled::open(index_height_path).ok()?;

        if let Ok(lookup) = index_height_db.get(&[0x00]) {
            if let Some(height) = lookup {
                index_height = u64::from_be_bytes(height.as_ref().try_into().ok()?);
            }
        };

        let sessions_path = format!("{}/{}/{}", "db/noist/dkgdir", batch_no, "dkgses");
        let sessions_db = sled::open(sessions_path).ok()?;

        let mut sessions = HashMap::<u64, DKGSession>::new();

        // Insert DKG sessions to the memory.
        for lookup in sessions_db.iter() {
            if let Ok((_, session)) = lookup {
                let session: DKGSession = match serde_json::from_slice(&session) {
                    Ok(session) => session,
                    Err(_) => return None,
                };

                sessions.insert(session.index(), session);
            }
        }

        Some(DKGDirectory {
            setup: setup.to_owned(),
            sessions,
            sessions_db,
            index_height,
            index_height_db,
        })
    }

    pub fn set_index_height(&mut self, new_height: u64) {
        if self.index_height < new_height {
            let _ = self
                .index_height_db
                .insert(&[0x00], &new_height.to_be_bytes());
            self.index_height = new_height;
        }
    }

    pub fn setup(&self) -> &VSESetup {
        &self.setup
    }

    pub fn sessions(&self) -> HashMap<u64, DKGSession> {
        self.sessions.clone()
    }

    pub fn sessions_db(&self) -> sled::Db {
        self.sessions_db.clone()
    }

    pub fn setup_no(&self) -> u64 {
        self.setup.setup_no()
    }

    pub fn index_height(&self) -> u64 {
        self.index_height
    }

    pub fn signatories(&self) -> Vec<Point> {
        self.setup.signatories()
    }

    pub fn group_key_session(&self) -> Option<DKGSession> {
        Some(self.sessions.get(&0)?.to_owned())
    }

    pub fn group_nonce_session(&self, nonce: u64) -> Option<DKGSession> {
        // Nonce '0' is allocated for the group key.
        if nonce == 0 {
            return None;
        }
        Some(self.sessions.get(&nonce)?.to_owned())
    }

    pub fn group_key(&self) -> Option<Point> {
        let group_key_session = self.group_key_session()?;
        let group_key = group_key_session.group_combined_full_point(None, None)?;
        Some(group_key)
    }

    pub fn group_nonce(&self, index: u64, message: [u8; 32]) -> Option<Point> {
        let nonce_session = self.group_nonce_session(index)?;
        let group_key_bytes = self.group_key()?.serialize_xonly();
        let group_nonce =
            nonce_session.group_combined_full_point(Some(group_key_bytes), Some(message))?;
        Some(group_nonce)
    }

    pub fn remove_session(&mut self, index: u64) -> bool {
        // Group key session cannot be removed.
        if index == 0 {
            return false;
        }

        match self.sessions_db.remove(index.to_be_bytes()) {
            Ok(removed) => {
                if let None = removed {
                    return false;
                }
            }
            Err(_) => return false,
        };

        if let None = self.sessions.remove(&index) {
            return false;
        }

        true
    }

    pub fn num_nonce_sessions(&self) -> u64 {
        (self.sessions.len() as u64) - 1 // Total number of sessions minus the key session.
    }

    pub fn new_session_to_fill(&mut self) -> Option<DKGSession> {
        let new_index_height = {
            match self.group_key_session() {
                None => 0,
                Some(_) => {
                    let new_index_height = self.index_height + 1;
                    self.set_index_height(new_index_height);
                    new_index_height
                }
            }
        };

        DKGSession::new(new_index_height, &self.signatories())
    }

    pub fn insert_session_filled(&mut self, session: &DKGSession) -> bool {
        // Nonce session insertions are not allowed, if key session is not set.

        let session_index = session.index();

        if let None = self.group_key_session() {
            if session_index != 0 {
                return false;
            }
        }

        if !session.verify(&self.setup) {
            return false;
        }

        if let Ok(_) = self
            .sessions_db
            .insert(session.index().to_be_bytes(), session.serialize())
        {
            if let None = self.sessions.insert(session_index, session.to_owned()) {
                self.set_index_height(session_index);
                return true;
            }
        }
        false
    }

    pub fn available_sessions(&self) -> u64 {
        self.sessions.len() as u64
    }

    fn pick_index(&mut self) -> Option<u64> {
        let index = self.sessions.keys().filter(|&&key| key != 0).min()?;
        Some(index.to_owned())
    }

    pub fn pick_signing_session(&mut self, message: [u8; 32]) -> Option<SigningSession> {
        let fresh_index = self.pick_index()?;

        let group_key_session = self.group_key_session()?;
        let group_nonce_session = self.group_nonce_session(fresh_index)?;

        let group_key = self.group_key()?;
        let group_nonce = self.group_nonce(group_nonce_session.index(), message)?;

        let signing_session = SigningSession::new(
            &group_key_session,
            &group_nonce_session,
            group_key,
            group_nonce,
            message,
        )?;

        self.remove_session(fresh_index);

        Some(signing_session)
    }
}

#[derive(Clone)]
pub struct SigningSession {
    group_key_session: DKGSession,
    group_nonce_session: DKGSession,
    group_key: Point,
    group_nonce: Point,
    message: [u8; 32],
    challenge: Scalar,
    partial_sigs: HashMap<Point, Scalar>,
}

impl SigningSession {
    pub fn new(
        group_key_session: &DKGSession,
        group_nonce_session: &DKGSession,
        group_key: Point,
        group_nonce: Point,
        message: [u8; 32],
    ) -> Option<SigningSession> {
        let challenge = match challenge(
            group_nonce,
            group_key,
            message,
            crate::schnorr::SigningMode::BIP340,
        ) {
            MaybeScalar::Valid(scalar) => scalar,
            MaybeScalar::Zero => return None,
        };

        let session = SigningSession {
            group_key_session: group_key_session.to_owned(),
            group_nonce_session: group_nonce_session.to_owned(),
            group_key,
            group_nonce,
            message,
            challenge,
            partial_sigs: HashMap::<Point, Scalar>::new(),
        };

        Some(session)
    }

    pub fn index(&self) -> u64 {
        self.group_nonce_session.index()
    }

    pub fn challenge(&self) -> Scalar {
        self.challenge
    }

    pub fn partial_sign(&self, secret_key: [u8; 32]) -> Option<Scalar> {
        let group_key_bytes = self.group_key.serialize_xonly();
        let message_bytes = self.message;
        let challenge = self.challenge;

        // (k + ed) + (k + ed)
        let hiding_secret_key_ = self
            .group_key_session
            .signatory_combined_hiding_secret(secret_key)?;
        let hiding_secret_key = hiding_secret_key_.negate_if(self.group_key.parity());

        let post_binding_secret_key_ = self
            .group_key_session
            .signatory_combined_post_binding_secret(secret_key, None, None)?;
        let post_binding_secret_key = post_binding_secret_key_.negate_if(self.group_key.parity());

        let hiding_secret_nonce_ = self
            .group_nonce_session
            .signatory_combined_hiding_secret(secret_key)?;
        let hiding_secret_nonce = hiding_secret_nonce_.negate_if(self.group_nonce.parity());

        let post_binding_secret_nonce_ = self
            .group_nonce_session
            .signatory_combined_post_binding_secret(
                secret_key,
                Some(group_key_bytes),
                Some(message_bytes),
            )?;
        let post_binding_secret_nonce =
            post_binding_secret_nonce_.negate_if(self.group_nonce.parity());

        let partial_sig = (hiding_secret_nonce + (challenge * hiding_secret_key))
            + (post_binding_secret_nonce + (challenge * post_binding_secret_key));

        match partial_sig {
            MaybeScalar::Valid(scalar) => return Some(scalar),
            MaybeScalar::Zero => return None,
        }
    }

    pub fn partial_sig_verify(&self, signatory: Point, sig: Scalar) -> bool {
        let group_key_bytes = self.group_key.serialize_xonly();
        let message_bytes = self.message;
        let challenge = self.challenge;

        // (R + eP) + (R + eP)
        let hiding_public_key_ = match self
            .group_key_session
            .signatory_combined_hiding_public(signatory)
        {
            Some(point) => point,
            None => return false,
        };

        let hiding_public_key = hiding_public_key_.negate_if(self.group_key.parity());

        let post_binding_public_key_ = match self
            .group_key_session
            .signatory_combined_post_binding_public(signatory, None, None)
        {
            Some(point) => point,
            None => return false,
        };

        let post_binding_public_key = post_binding_public_key_.negate_if(self.group_key.parity());

        let hiding_public_nonce_ = match self
            .group_nonce_session
            .signatory_combined_hiding_public(signatory)
        {
            Some(point) => point,
            None => return false,
        };

        let hiding_public_nonce = hiding_public_nonce_.negate_if(self.group_nonce.parity());

        let post_binding_public_nonce_ = match self
            .group_nonce_session
            .signatory_combined_post_binding_public(
                signatory,
                Some(group_key_bytes),
                Some(message_bytes),
            ) {
            Some(point) => point,
            None => return false,
        };

        let post_binding_public_nonce =
            post_binding_public_nonce_.negate_if(self.group_nonce.parity());

        let equation = (hiding_public_nonce + (challenge * hiding_public_key))
            + (post_binding_public_nonce + (challenge * post_binding_public_key));

        let equation_point = match equation {
            MaybePoint::Valid(point) => point,
            MaybePoint::Infinity => return false,
        };

        equation_point == sig.base_point_mul()
    }

    pub fn insert_partial_sig(&mut self, signatory: Point, sig: Scalar) -> bool {
        if self.partial_sig_verify(signatory, sig) {
            if let None = self.partial_sigs.insert(signatory, sig) {
                return true;
            }
        }
        false
    }

    pub fn is_above_threshold(&self) -> bool {
        let threshold = (self.group_key_session.signatories().len() / 2) + 1;
        self.partial_sigs.len() >= threshold
    }

    pub fn aggregated_sig(&self) -> Option<Scalar> {
        let full_list = self.group_key_session.signatories();
        let mut active_list = Vec::<Point>::new();

        // Fill lagrance index list.
        for (signatory, _) in self.partial_sigs.iter() {
            active_list.push(signatory.to_owned());
        }

        let index_list = lagrance_index_list(&full_list, &active_list)?;

        let mut agg_sig = MaybeScalar::Zero;

        for (signatory, partial_sig) in self.partial_sigs.iter() {
            let lagrance_index = lagrance_index(&full_list, signatory.to_owned())?;
            let lagrance = interpolating_value(&index_list, lagrance_index).ok()?;
            let partial_sig_lagranced = partial_sig.to_owned() * lagrance;
            agg_sig = agg_sig + partial_sig_lagranced;
        }

        match agg_sig {
            MaybeScalar::Valid(scalar) => return Some(scalar),
            MaybeScalar::Zero => return None,
        };
    }

    pub fn full_aggregated_sig_bytes(&self) -> Option<[u8; 64]> {
        let group_nonce = self.group_nonce.serialize_xonly();
        let agg_sig = self.aggregated_sig()?.serialize();

        let mut full = Vec::<u8>::with_capacity(64);
        full.extend(group_nonce);
        full.extend(agg_sig);
        let full_bytes: [u8; 64] = full.try_into().ok()?;
        Some(full_bytes)
    }
}
