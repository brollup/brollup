use super::session::DKGSession;
use crate::{
    into::IntoScalar,
    musig::{nesting::MusigNestingCtx, session::MusigSessionCtx},
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
        let setup_height = setup.height();
        // sessions path 'db/signatory/dkg/batches/BATCH_NO/sessions' key is SESSION_INDEX
        // manager path 'db/signatory/dkg/batches/manager' key is BATCH_NO
        let mut index_height: u64 = 0;

        let index_height_path = format!("{}/{}", "db/noist/dkgdir/", setup_height);
        let index_height_db = sled::open(index_height_path).ok()?;

        if let Ok(lookup) = index_height_db.get(&[0x00]) {
            if let Some(height) = lookup {
                index_height = u64::from_be_bytes(height.as_ref().try_into().ok()?);
            }
        };

        let sessions_path = format!("{}/{}/{}", "db/noist/dkgdir", setup_height, "dkgses");
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

    pub fn setup(&self) -> VSESetup {
        self.setup.clone()
    }

    pub fn sessions(&self) -> HashMap<u64, DKGSession> {
        self.sessions.clone()
    }

    pub fn sessions_db(&self) -> sled::Db {
        self.sessions_db.clone()
    }

    pub fn setup_height(&self) -> u64 {
        self.setup.height()
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

        if let Some(_) = self.sessions.get(&session_index) {
            return false;
        }

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

    pub fn pick_index(&self) -> Option<u64> {
        let index = self.sessions.keys().filter(|&&key| key != 0).min()?;
        Some(index.to_owned())
    }

    pub fn pick_signing_session(
        &mut self,
        message: [u8; 32],
        musig_nesting_ctx: Option<MusigNestingCtx>,
    ) -> Option<SigningSession> {
        self.signing_session(message, self.pick_index()?, musig_nesting_ctx)
    }

    pub fn signing_session(
        &mut self,
        message: [u8; 32],
        nonce_index: u64,
        musig_nesting_ctx: Option<MusigNestingCtx>,
    ) -> Option<SigningSession> {
        let group_key_session = self.group_key_session()?;

        let group_nonce_session = self.group_nonce_session(nonce_index)?;

        let group_key = self.group_key()?;

        let hiding_group_nonce = group_nonce_session.group_combined_hiding_point()?;
        let post_binding_group_nonce = group_nonce_session
            .group_combined_post_binding_point(Some(group_key.serialize_xonly()), Some(message))?;

        let group_nonce = self.group_nonce(nonce_index, message)?;

        let signing_session = SigningSession::new(
            &group_key_session,
            &group_nonce_session,
            group_key,
            hiding_group_nonce,
            post_binding_group_nonce,
            group_nonce,
            message,
            musig_nesting_ctx,
        )?;

        self.remove_session(nonce_index);

        Some(signing_session)
    }
}

#[derive(Clone)]
pub struct SigningSession {
    pub group_key_session: DKGSession,
    pub group_nonce_session: DKGSession,
    pub group_key: Point,
    pub hiding_group_nonce: Point,
    pub post_binding_group_nonce: Point,
    pub group_nonce: Point,
    pub message: [u8; 32],
    pub challenge: Scalar,
    pub musig_ctx: Option<MusigSessionCtx>,
    partial_sigs: HashMap<Point, Scalar>,
}

impl SigningSession {
    pub fn new(
        group_key_session: &DKGSession,
        group_nonce_session: &DKGSession,
        group_key: Point,
        hiding_group_nonce: Point,
        post_binding_group_nonce: Point,
        group_nonce: Point,
        message: [u8; 32],
        musig_nesting_ctx: Option<MusigNestingCtx>,
    ) -> Option<SigningSession> {
        let musig_ctx = match &musig_nesting_ctx {
            Some(ctx) => {
                let mut musig_ctx = ctx.musig_ctx(
                    group_key,
                    hiding_group_nonce,
                    post_binding_group_nonce,
                    message,
                )?;

                if !musig_ctx.ready() {
                    return None;
                }

                Some(musig_ctx)
            }
            None => None,
        };

        let challenge = match &musig_ctx {
            Some(ctx) => ctx.challenge()?,
            None => match challenge(
                group_nonce,
                group_key,
                message,
                crate::schnorr::SigningMode::BIP340,
            ) {
                MaybeScalar::Valid(scalar) => scalar,
                MaybeScalar::Zero => return None,
            },
        };

        let session = SigningSession {
            group_key_session: group_key_session.to_owned(),
            group_nonce_session: group_nonce_session.to_owned(),
            group_key,
            hiding_group_nonce,
            post_binding_group_nonce,
            group_nonce,
            message,
            challenge,
            musig_ctx,
            partial_sigs: HashMap::<Point, Scalar>::new(),
        };

        Some(session)
    }

    pub fn musig_ctx(&self) -> Option<MusigSessionCtx> {
        self.musig_ctx.clone()
    }

    pub fn nonce_index(&self) -> u64 {
        self.group_nonce_session.index()
    }

    pub fn challenge(&self) -> Scalar {
        self.challenge
    }

    pub fn partial_sign(&self, secret_key: [u8; 32]) -> Option<Scalar> {
        let public_key = secret_key.into_scalar().ok()?.base_point_mul();

        let group_key_bytes = self.group_key.serialize_xonly();
        let message_bytes = self.message;
        let challenge = self.challenge;

        let compare_key = match &self.musig_ctx {
            Some(ctx) => ctx.key_agg_ctx().agg_inner_key(),
            None => self.group_key,
        };

        let compare_nonce = match &self.musig_ctx {
            Some(ctx) => ctx.agg_nonce()?,
            None => self.group_nonce,
        };

        let musig_nonce_coef = match &self.musig_ctx {
            Some(ctx) => ctx.nonce_coef()?,
            None => Scalar::one(),
        };

        let musig_key_coef = match &self.musig_ctx {
            Some(ctx) => ctx.key_agg_ctx().key_coef(self.group_key)?,
            None => Scalar::one(),
        };

        // (k + ed) + (k + ed)

        let hiding_secret_key_ = self
            .group_key_session
            .signatory_combined_hiding_secret(secret_key)?
            * musig_key_coef;

        let mut hiding_secret_key = hiding_secret_key_.negate_if(compare_key.parity());

        if let Some(ctx) = &self.musig_ctx {
            if let Some(_) = ctx.key_agg_ctx().tweak() {
                hiding_secret_key =
                    hiding_secret_key.negate_if(ctx.key_agg_ctx().agg_key().parity())
            }
        }

        let post_binding_secret_key_ = self
            .group_key_session
            .signatory_combined_post_binding_secret(secret_key, None, None)?
            * musig_key_coef;

        let mut post_binding_secret_key = post_binding_secret_key_.negate_if(compare_key.parity());

        if let Some(ctx) = &self.musig_ctx {
            if let Some(_) = ctx.key_agg_ctx().tweak() {
                post_binding_secret_key =
                    post_binding_secret_key.negate_if(ctx.key_agg_ctx().agg_key().parity())
            }
        }

        let hiding_secret_nonce_ = self
            .group_nonce_session
            .signatory_combined_hiding_secret(secret_key)?;
        let hiding_secret_nonce = hiding_secret_nonce_.negate_if(compare_nonce.parity());

        let post_binding_secret_nonce_ = self
            .group_nonce_session
            .signatory_combined_post_binding_secret(
                secret_key,
                Some(group_key_bytes),
                Some(message_bytes),
            )?
            * musig_nonce_coef;

        let post_binding_secret_nonce =
            post_binding_secret_nonce_.negate_if(compare_nonce.parity());

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

        let compare_key = match &self.musig_ctx {
            Some(ctx) => ctx.key_agg_ctx().agg_inner_key(),
            None => self.group_key,
        };

        let compare_nonce = match &self.musig_ctx {
            Some(ctx) => match ctx.agg_nonce() {
                Some(nonce) => nonce,
                None => return false,
            },
            None => self.group_nonce,
        };

        let musig_nonce_coef = match &self.musig_ctx {
            Some(ctx) => match ctx.nonce_coef() {
                Some(coef) => coef,
                None => return false,
            },
            None => Scalar::one(),
        };

        let musig_key_coef = match &self.musig_ctx {
            Some(ctx) => match ctx.key_agg_ctx().key_coef(self.group_key) {
                Some(coef) => coef,
                None => return false,
            },
            None => Scalar::one(),
        };

        // (R + eP) + (R + eP)

        let hiding_public_key_ = match self
            .group_key_session
            .signatory_combined_hiding_public(signatory)
        {
            Some(point) => point,
            None => return false,
        } * musig_key_coef;

        let mut hiding_public_key = hiding_public_key_.negate_if(compare_key.parity());

        if let Some(ctx) = &self.musig_ctx {
            if let Some(_) = ctx.key_agg_ctx().tweak() {
                hiding_public_key =
                    hiding_public_key.negate_if(ctx.key_agg_ctx().agg_key().parity())
            }
        }

        let post_binding_public_key_ = match self
            .group_key_session
            .signatory_combined_post_binding_public(signatory, None, None)
        {
            Some(point) => point,
            None => return false,
        } * musig_key_coef;

        let mut post_binding_public_key = post_binding_public_key_.negate_if(compare_key.parity());

        if let Some(ctx) = &self.musig_ctx {
            if let Some(_) = ctx.key_agg_ctx().tweak() {
                post_binding_public_key =
                    post_binding_public_key.negate_if(ctx.key_agg_ctx().agg_key().parity())
            }
        }

        let hiding_public_nonce_ = match self
            .group_nonce_session
            .signatory_combined_hiding_public(signatory)
        {
            Some(point) => point,
            None => return false,
        };

        let hiding_public_nonce = hiding_public_nonce_.negate_if(compare_nonce.parity());

        let post_binding_public_nonce_ = match self
            .group_nonce_session
            .signatory_combined_post_binding_public(
                signatory,
                Some(group_key_bytes),
                Some(message_bytes),
            ) {
            Some(point) => point,
            None => return false,
        } * musig_nonce_coef;

        let post_binding_public_nonce =
            post_binding_public_nonce_.negate_if(compare_nonce.parity());

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

    pub fn is_threshold_met(&self) -> bool {
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
