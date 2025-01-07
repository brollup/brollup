use super::session::DKGSession;
use crate::{
    noist::{
        lagrance::{interpolating_value, lagrance_index, lagrance_index_list},
        setup::setup::VSESetup,
    },
    schnorr::challenge,
    secp_point::SecpPoint,
};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};
use std::collections::HashMap;

#[derive(Clone)]
pub struct DKGDirectory {
    batch_no: u64,                      // In-memory batch number.
    vse_setup: VSESetup,                // VSE setup.
    signatories: Vec<SecpPoint>,        // Signatories list.
    sessions: HashMap<u64, DKGSession>, // In-memory DKG sessions (nonce, (session, is_toxic)).
    sessions_db: sled::Db,              // Database connection.
    nonce_bound: u64,
}
// 'db/signatory/dkg/batches/BATCH_NO' key:SESSION_NONCE
impl DKGDirectory {
    pub fn new(batch_no: u64, signatories: &Vec<Point>, vse_setup: &VSESetup) -> Option<Self> {
        if vse_setup.no() != batch_no {
            return None;
        }

        let mut signatories = signatories.clone();
        signatories.sort();

        let signatories_ = signatories.into_iter().map(SecpPoint::new).collect();

        // sessions path 'db/signatory/dkg/batches/BATCH_NO/sessions' key is SESSION_INDEX
        // manager path 'db/signatory/dkg/batches/manager' key is BATCH_NO
        let sessions_path = format!("{}/{}/sessions", "db/signatory/dkg/batches", batch_no);
        let sessions_db = sled::open(sessions_path).ok()?;

        let mut sessions = HashMap::<u64, DKGSession>::new();

        // Insert DKG sessions to the memory.
        for lookup in sessions_db.iter() {
            if let Ok((nonce, session)) = lookup {
                let nonce: u64 = u64::from_be_bytes(nonce.as_ref().try_into().ok()?);

                let session: DKGSession = match bincode::deserialize(&session) {
                    Ok(session) => session,
                    Err(_) => return None,
                };

                sessions.insert(nonce, session);
            }
        }

        let nonce_bound = sessions.keys().max().unwrap_or(&0).to_owned();

        Some(DKGDirectory {
            batch_no,
            vse_setup: vse_setup.to_owned(),
            signatories: signatories_,
            sessions,
            sessions_db,
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

    pub fn sessions(&self) -> HashMap<u64, DKGSession> {
        self.sessions.clone()
    }

    pub fn nonce_bound(&self) -> u64 {
        self.nonce_bound
    }

    pub fn signatories(&self) -> Vec<Point> {
        self.signatories
            .iter()
            .map(|secp_point| secp_point.inner().clone())
            .collect()
    }

    pub fn insert(&mut self, session: &DKGSession) -> bool {
        // Nonce session insertions are not allowed, if key session is not set.

        let session_nonce = session.nonce();

        if let None = self.key_session() {
            if session_nonce != 0 {
                return false;
            }
        } else {
            if session_nonce < self.nonce_bound {
                println!("hee burda");
                return false;
            }
        }

        if !session.verify(&self.vse_setup) {
            return false;
        }

        if let Ok(_) = self
            .sessions_db
            .insert(session.nonce().to_be_bytes(), session.serialize())
        {
            if let None = self.sessions.insert(session_nonce, session.to_owned()) {
                self.nonce_bound = session_nonce;
                return true;
            }
        }
        false
    }

    pub fn key_session(&self) -> Option<DKGSession> {
        Some(self.sessions.get(&0)?.to_owned())
    }

    pub fn nonce_session(&self, nonce: u64) -> Option<DKGSession> {
        // Nonce '0' is allocated for the group key.
        if nonce == 0 {
            return None;
        }
        Some(self.sessions.get(&nonce)?.to_owned())
    }

    pub fn group_key(&self) -> Option<Point> {
        let group_key_session = self.key_session()?;
        let group_key = group_key_session.group_combined_full_point(None, None)?;
        Some(group_key)
    }

    pub fn group_nonce(&self, nonce: u64, message: [u8; 32]) -> Option<Point> {
        let nonce_session = self.nonce_session(nonce)?;
        let group_key_bytes = self.group_key()?.serialize_xonly();
        let group_nonce =
            nonce_session.group_combined_full_point(Some(group_key_bytes), Some(message))?;
        Some(group_nonce)
    }

    pub fn challenge(&self, nonce: u64, message: [u8; 32]) -> Option<Scalar> {
        let group_nonce = self.group_nonce(nonce, message)?;
        let group_key = self.group_key()?;

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

    pub fn remove(&mut self, nonce: u64) -> bool {
        // Group key session cannot be removed.
        if nonce == 0 {
            return false;
        }

        if let Err(_) = self.sessions_db.remove(nonce.to_be_bytes()) {
            return false;
        }

        if let None = self.sessions.remove(&nonce) {
            return false;
        }

        true
    }

    pub fn num_nonce_sessions(&self) -> u64 {
        (self.sessions.len() as u64) - 1 // Total number of sessions minus the key session.
    }

    pub fn new_session(&mut self) -> Option<DKGSession> {
        let new_nonce_bound = {
            match self.key_session() {
                None => 0,
                Some(_) => {
                    let new_nonce_bound = self.nonce_bound + 1;
                    self.nonce_bound = new_nonce_bound;
                    new_nonce_bound
                }
            }
        };

        DKGSession::new(new_nonce_bound, &self.signatories())
    }

    pub fn pick_nonce(&mut self) -> Option<u64> {
        self.sessions
            .iter()
            .filter(|(&key, _)| key != 0)
            .max_by_key(|(&key, _)| key)
            .map(|(&key, _)| key)
    }

    pub fn signing_session(&mut self, nonce: u64, message: [u8; 32]) -> Option<SigningSession> {
        let key_session = self.key_session()?;
        let nonce_session = self.nonce_session(nonce)?;

        let challenge = self.challenge(nonce_session.nonce(), message)?;
        let group_key = self.group_key()?;
        let group_nonce = self.group_nonce(nonce_session.nonce(), message)?;

        let signing_session = SigningSession::new(
            &key_session,
            &nonce_session,
            challenge,
            message,
            group_key,
            group_nonce,
        );

        self.remove(nonce);

        Some(signing_session)
    }
}

#[derive(Clone)]
pub struct SigningSession {
    key_session: DKGSession,
    nonce_session: DKGSession,
    challenge: Scalar,
    message: [u8; 32],
    group_key: Point,
    group_nonce: Point,
    partial_sigs: HashMap<Point, Scalar>,
}

impl SigningSession {
    pub fn new(
        key_session: &DKGSession,
        nonce_session: &DKGSession,
        challenge: Scalar,
        message: [u8; 32],
        group_key: Point,
        group_nonce: Point,
    ) -> SigningSession {
        SigningSession {
            key_session: key_session.to_owned(),
            nonce_session: nonce_session.to_owned(),
            challenge,
            message,
            group_key,
            group_nonce,
            partial_sigs: HashMap::<Point, Scalar>::new(),
        }
    }

    pub fn partial_sign(&self, secret_key: [u8; 32]) -> Option<Scalar> {
        let group_key_bytes = self.group_key.serialize_xonly();
        let message_bytes = self.message;
        let challenge = self.challenge;

        // (k + ed) + (k + ed)
        let hiding_secret_key_ = self
            .key_session
            .signatory_combined_hiding_secret(secret_key)?;
        let hiding_secret_key = hiding_secret_key_.negate_if(self.group_key.parity());

        let post_binding_secret_key_ = self
            .key_session
            .signatory_combined_post_binding_secret(secret_key, None, None)?;
        let post_binding_secret_key = post_binding_secret_key_.negate_if(self.group_key.parity());

        let hiding_secret_nonce_ = self
            .nonce_session
            .signatory_combined_hiding_secret(secret_key)?;
        let hiding_secret_nonce = hiding_secret_nonce_.negate_if(self.group_nonce.parity());

        let post_binding_secret_nonce_ =
            self.nonce_session.signatory_combined_post_binding_secret(
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
        let hiding_public_key_ = match self.key_session.signatory_combined_hiding_public(signatory)
        {
            Some(point) => point,
            None => return false,
        };

        let hiding_public_key = hiding_public_key_.negate_if(self.group_key.parity());

        let post_binding_public_key_ = match self
            .key_session
            .signatory_combined_post_binding_public(signatory, None, None)
        {
            Some(point) => point,
            None => return false,
        };

        let post_binding_public_key = post_binding_public_key_.negate_if(self.group_key.parity());

        let hiding_public_nonce_ = match self
            .nonce_session
            .signatory_combined_hiding_public(signatory)
        {
            Some(point) => point,
            None => return false,
        };

        let hiding_public_nonce = hiding_public_nonce_.negate_if(self.group_nonce.parity());

        let post_binding_public_nonce_ =
            match self.nonce_session.signatory_combined_post_binding_public(
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
        let threshold = (self.key_session.signatories().len() / 2) + 1;
        self.partial_sigs.len() >= threshold
    }

    pub fn aggregated_sig(&self) -> Option<Scalar> {
        let full_list = self.key_session.signatories();
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
