use super::{
    commiterr::CSessionCommitError, opcov::CSessionOpCov, opcovack::OSessionOpCovAck,
    uphold::NSessionUphold, upholderr::CSessionUpholdError,
};
use crate::{
    entry::{call::Call, liftup::Liftup, recharge::Recharge, reserved::Reserved, vanilla::Vanilla},
    musig::{keyagg::MusigKeyAggCtx, session::MusigSessionCtx},
    noist::session::NOISTSessionCtx,
    registery::account::account_registery_index,
    schnorr::Authenticable,
    session::{allowance::allowance, commit::NSessionCommit, commitack::CSessionCommitAck},
    txo::{
        connector::Connector,
        lift::Lift,
        projector::{self, Projector},
    },
    valtype::account::Account,
    CSESSION_CTX, DKG_DIRECTORY, DKG_MANAGER,
};
use secp::Point;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type DKGDirHeight = u64;
type DKGNonceHeight = u64;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CSessionStage {
    On,        // The session is on. New commitments are now allowed to join.
    Locked,    // The session is locked. No new commitments are allowed.
    Upheld,    // The session is upheld. All partial MuSig signatures are collected.
    Finalized, // The session is finalized. All forfeiture signatures are collected.
    Off,       // The session is off.
}

#[derive(Clone)]
pub struct CSessionCtx {
    dkg_manager: DKG_MANAGER,
    stage: CSessionStage,
    // Remote keys
    msg_senders: Vec<Account>,
    // Liftups
    liftups: Vec<Liftup>,
    // Recharges
    recharges: Vec<Recharge>,
    // Vanillas
    vanillas: Vec<Vanilla>,
    // Calls
    calls: Vec<Call>,
    // Reserveds:
    reserveds: Vec<Reserved>,
    // Payload Auth:
    payload_auth_nonces: HashMap<Account, (Point, Point)>,
    payload_auth_ctxes: Option<(
        DKGDirHeight,
        DKGNonceHeight,
        NOISTSessionCtx,
        MusigSessionCtx,
    )>,
    // VTXO projector:
    vtxo_projector_nonces: HashMap<Account, (Point, Point)>,
    vtxo_projector_ctxes: Option<(
        DKGDirHeight,
        DKGNonceHeight,
        NOISTSessionCtx,
        MusigSessionCtx,
    )>,
    // Connector projector:
    connector_projector_nonces: HashMap<Account, (Point, Point)>,
    connector_projector_ctxes: Option<(
        DKGDirHeight,
        DKGNonceHeight,
        NOISTSessionCtx,
        MusigSessionCtx,
    )>,
    // ZKP contingent:
    zkp_contingent_nonces: HashMap<Account, (Point, Point)>,
    zkp_contingent_ctxes: Option<(
        DKGDirHeight,
        DKGNonceHeight,
        NOISTSessionCtx,
        MusigSessionCtx,
    )>,
    // Lift txos:
    lift_prevtxo_nonces: HashMap<Account, HashMap<Lift, (Point, Point)>>,
    lift_prevtxo_ctxes: HashMap<
        Account,
        HashMap<
            Lift,
            (
                DKGDirHeight,
                DKGNonceHeight,
                NOISTSessionCtx,
                MusigSessionCtx,
            ),
        >,
    >,
    // Connectors:
    connector_txo_nonces: HashMap<Account, Vec<(Point, Point)>>,
    connector_txo_ctxes: HashMap<
        Account,
        Vec<(
            DKGDirHeight,
            DKGNonceHeight,
            NOISTSessionCtx,
            MusigSessionCtx,
        )>,
    >,
}

impl CSessionCtx {
    pub fn construct(dkg_manager: &DKG_MANAGER) -> CSESSION_CTX {
        let session = CSessionCtx {
            dkg_manager: Arc::clone(dkg_manager),
            stage: CSessionStage::Off,
            msg_senders: Vec::<Account>::new(),
            liftups: Vec::<Liftup>::new(),
            recharges: Vec::<Recharge>::new(),
            vanillas: Vec::<Vanilla>::new(),
            calls: Vec::<Call>::new(),
            reserveds: Vec::<Reserved>::new(),
            payload_auth_nonces: HashMap::<Account, (Point, Point)>::new(),
            payload_auth_ctxes: None,
            vtxo_projector_nonces: HashMap::<Account, (Point, Point)>::new(),
            vtxo_projector_ctxes: None,
            connector_projector_nonces: HashMap::<Account, (Point, Point)>::new(),
            connector_projector_ctxes: None,
            zkp_contingent_nonces: HashMap::<Account, (Point, Point)>::new(),
            zkp_contingent_ctxes: None,
            lift_prevtxo_nonces: HashMap::<Account, HashMap<Lift, (Point, Point)>>::new(),
            lift_prevtxo_ctxes: HashMap::<
                Account,
                HashMap<
                    Lift,
                    (
                        DKGDirHeight,
                        DKGNonceHeight,
                        NOISTSessionCtx,
                        MusigSessionCtx,
                    ),
                >,
            >::new(),
            connector_txo_nonces: HashMap::<Account, Vec<(Point, Point)>>::new(),
            connector_txo_ctxes: HashMap::<
                Account,
                Vec<(
                    DKGDirHeight,
                    DKGNonceHeight,
                    NOISTSessionCtx,
                    MusigSessionCtx,
                )>,
            >::new(),
        };
        Arc::new(Mutex::new(session))
    }

    pub fn stage(&self) -> CSessionStage {
        self.stage
    }

    pub fn msg_senders_len(&self) -> usize {
        self.msg_senders.len()
    }

    pub fn msg_senders(&self) -> Vec<Account> {
        self.msg_senders.clone()
    }

    /// Sets the coordinator session stage to `on`
    /// meaning accounts can now participate in a rollup state transition.
    pub fn on(&mut self) {
        self.reset();
        self.stage = CSessionStage::On;
    }

    /// Validates a given `NSessionCommit` to ensure that the involved account
    /// and transactions are eligible for the rollup state transition.
    async fn validate_commit(
        &self,
        auth_commit: &Authenticable<NSessionCommit>,
    ) -> Result<(), CSessionCommitError> {
        let dkg_manager: DKG_MANAGER = Arc::clone(&self.dkg_manager);

        if !auth_commit.authenticate() {
            return Err(CSessionCommitError::AuthErr);
        }

        let commit = auth_commit.object();
        let msg_sender = commit.msg_sender();

        if auth_commit.key() != msg_sender.key().serialize_xonly() {
            return Err(CSessionCommitError::AuthErr);
        }

        // #1 Overlap check
        if self.msg_senders.contains(&msg_sender) {
            return Err(CSessionCommitError::Overlap);
        }

        // #2 Allowance check
        if allowance(msg_sender) {
            return Err(CSessionCommitError::Allowance);
        }

        // #3 Lift prevtxouts validation
        for (lift, _) in commit.lift_prevtxo_nonces().iter() {
            // #1 Operator key validation
            {
                let lift_operator_key = lift.operator_key();
                let lift_remote_key = lift.remote_key();

                if lift_operator_key == lift_remote_key {
                    return Err(CSessionCommitError::InvalidLiftRemoteKey);
                }

                if lift_remote_key != msg_sender.key() {
                    return Err(CSessionCommitError::InvalidLiftRemoteKey);
                }

                {
                    let dkg_manager_ = dkg_manager.lock().await;
                    if let None = dkg_manager_.directory_by_key(lift_operator_key).await {
                        return Err(CSessionCommitError::InvalidLiftOperatorKey);
                    }
                }
            }

            // #2 Outpoint validation
            {
                match lift.outpoint() {
                    Some(_outpoint) => {
                        // TODO: check if this is a valid outpoint.
                    }
                    None => return Err(CSessionCommitError::InvalidLiftOutpoint),
                };
            }
        }

        // #4 TODO: Check for num of connectors:
        let _connector_count = self.connector_projector_nonces.len();

        Ok(())
    }

    /// Attempts to insert an `NSessionCommit` into the coordinator session.  
    /// The coordinator awaits `NSessionCommit`s until the `CSessionStage` is set to `on`,  
    /// after which it inserts `NSessionCommit`s into the session context.  
    /// Shortly after, `CSessionStage` is set to `locked`, preventing further insertions for this session.
    pub async fn insert_commit(
        &mut self,
        auth_commit: &Authenticable<NSessionCommit>,
    ) -> Result<(), CSessionCommitError> {
        // #1 Check stage
        if self.stage != CSessionStage::On {
            return Err(CSessionCommitError::SessionLocked);
        }

        // #2 Validate commit
        self.validate_commit(&auth_commit).await?;

        let commit = auth_commit.object();
        let mut msg_sender = commit.msg_sender();

        // #3 Set registery index (if not set)
        if let Some(registery_index) = account_registery_index(msg_sender.key()) {
            msg_sender.set_registery_index(registery_index);
        }

        // #4 Insert into msg_senders
        self.msg_senders.push(msg_sender);

        // #5 Insert into liftups
        if let Some(liftup) = commit.liftup() {
            self.liftups.push(liftup);
        }

        // #6 Insert into recharges
        if let Some(recharge) = commit.recharge() {
            self.recharges.push(recharge);
        }

        // #7 Insert into vanillas
        if let Some(vanilla) = commit.vanilla() {
            self.vanillas.push(vanilla);
        }

        // #8 Insert into calls
        if let Some(call) = commit.call() {
            self.calls.push(call);
        }

        // #9 Insert into reserveds
        if let Some(reserved) = commit.reserved() {
            self.reserveds.push(reserved);
        }

        // #10 Insert payload auth nonce commitments
        let payload_auth_nonces = commit.payload_auth_nonces();
        self.payload_auth_nonces
            .insert(msg_sender, payload_auth_nonces);

        // #11 Insert vtxo projector nonce commitments
        let vtxo_projector_nonces = commit.vtxo_projector_nonces();
        self.vtxo_projector_nonces
            .insert(msg_sender, vtxo_projector_nonces);

        // #12 Insert connector projector nonce commitments
        let connector_projector_nonces = commit.connector_projector_nonces();
        self.connector_projector_nonces
            .insert(msg_sender, connector_projector_nonces);

        // #13 Insert zkp contingent nonce commitments
        let zkp_contingent_nonces = commit.zkp_contingent_nonces();
        self.zkp_contingent_nonces
            .insert(msg_sender, zkp_contingent_nonces);

        // #14 Insert lift nonce commitments
        let lift_prevtxo_nonces = commit.lift_prevtxo_nonces();
        self.lift_prevtxo_nonces
            .insert(msg_sender, lift_prevtxo_nonces);

        // #15 Insert connector nonce commitments
        let connector_txo_nonces = commit.connector_txo_nonces();
        self.connector_txo_nonces
            .insert(msg_sender, connector_txo_nonces);

        Ok(())
    }

    /// Sets the NOIST and MuSig contexes upon collecting `NSessionCommit`s, triggered by `lock`.
    async fn set_ctxes(&mut self) -> bool {
        let dkg_manager: DKG_MANAGER = Arc::clone(&self.dkg_manager);

        let active_dkg_dir: DKG_DIRECTORY = {
            let dkg_manager_ = dkg_manager.lock().await;
            match dkg_manager_.active_directory() {
                Some(dir) => dir,
                None => return false,
            }
        };

        let active_dir_height = {
            let active_dkg_dir_ = active_dkg_dir.lock().await;
            active_dkg_dir_.dir_height()
        };

        let payload_auth_ctxes = match self.payload_auth_nonces.len() {
            0 => return false,
            _ => {
                // TODO:
                let payload_auth_message = [0xffu8; 32];

                let mut dkg_dir_ = active_dkg_dir.lock().await;

                let noist_ctx =
                    match dkg_dir_.pick_signing_session(payload_auth_message, None, true) {
                        Some(session) => session,
                        None => return false,
                    };

                let nonce_height = noist_ctx.nonce_height();
                let mut keys: Vec<Point> = self
                    .payload_auth_nonces
                    .iter()
                    .map(|(account, _)| account.key().clone())
                    .collect();

                keys.push(noist_ctx.group_key());

                let key_agg_ctx = match MusigKeyAggCtx::new(&keys, None) {
                    Some(ctx) => ctx,
                    None => return false,
                };

                let mut musig_ctx = match MusigSessionCtx::new(&key_agg_ctx, payload_auth_message) {
                    Some(ctx) => ctx,
                    None => return false,
                };

                for (msg_sender, (hiding, binding)) in self.payload_auth_nonces.iter() {
                    if !musig_ctx.insert_nonce(
                        msg_sender.key(),
                        hiding.to_owned(),
                        binding.to_owned(),
                    ) {
                        return false;
                    }
                }

                let operator_key = noist_ctx.group_key();
                let operator_hiding = noist_ctx.hiding_group_nonce();
                let operator_binding = noist_ctx.post_binding_group_nonce();

                if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                    return false;
                }

                Some((active_dir_height, nonce_height, noist_ctx, musig_ctx))
            }
        };

        let vtxo_projector_ctxes = match self.vtxo_projector_nonces.len() {
            0 => None,
            _ => {
                // TODO:
                let vtxo_projector_message = [0xffu8; 32];

                let mut dkg_dir_ = active_dkg_dir.lock().await;

                let noist_ctx =
                    match dkg_dir_.pick_signing_session(vtxo_projector_message, None, true) {
                        Some(session) => session,
                        None => return false,
                    };

                let nonce_height = noist_ctx.nonce_height();

                let remote_keys: Vec<Point> = self
                    .vtxo_projector_nonces
                    .iter()
                    .map(|(account, _)| account.key().clone())
                    .collect();

                let operator_key = noist_ctx.group_key();

                let vtxo_projector = Projector::new(
                    &remote_keys,
                    operator_key,
                    projector::ProjectorTag::VTXOProjector,
                );

                let key_agg_ctx = match vtxo_projector.key_agg_ctx() {
                    Some(ctx) => ctx,
                    None => return false,
                };

                let mut musig_ctx = match MusigSessionCtx::new(&key_agg_ctx, vtxo_projector_message)
                {
                    Some(ctx) => ctx,
                    None => return false,
                };

                for (msg_sender, (hiding, binding)) in self.vtxo_projector_nonces.iter() {
                    if !musig_ctx.insert_nonce(
                        msg_sender.key(),
                        hiding.to_owned(),
                        binding.to_owned(),
                    ) {
                        return false;
                    }
                }

                let operator_key = noist_ctx.group_key();
                let operator_hiding = noist_ctx.hiding_group_nonce();
                let operator_binding = noist_ctx.post_binding_group_nonce();

                if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                    return false;
                }

                Some((active_dir_height, nonce_height, noist_ctx, musig_ctx))
            }
        };

        let connector_projector_ctxes = match self.connector_projector_nonces.len() {
            0 => None,
            _ => {
                // TODO:
                let connector_projector_message = [0xffu8; 32];

                let mut dkg_dir_ = active_dkg_dir.lock().await;

                let noist_ctx =
                    match dkg_dir_.pick_signing_session(connector_projector_message, None, true) {
                        Some(session) => session,
                        None => return false,
                    };

                let nonce_height = noist_ctx.nonce_height();
                let remote_keys: Vec<Point> = self
                    .connector_projector_nonces
                    .iter()
                    .map(|(account, _)| account.key().clone())
                    .collect();

                let operator_key = noist_ctx.group_key();

                let connector_projector = Projector::new(
                    &remote_keys,
                    operator_key,
                    projector::ProjectorTag::ConnectorProjector,
                );

                let key_agg_ctx = match connector_projector.key_agg_ctx() {
                    Some(ctx) => ctx,
                    None => return false,
                };

                let mut musig_ctx =
                    match MusigSessionCtx::new(&key_agg_ctx, connector_projector_message) {
                        Some(ctx) => ctx,
                        None => return false,
                    };

                for (msg_sender, (hiding, binding)) in self.connector_projector_nonces.iter() {
                    if !musig_ctx.insert_nonce(
                        msg_sender.key(),
                        hiding.to_owned(),
                        binding.to_owned(),
                    ) {
                        return false;
                    }
                }

                let operator_key = noist_ctx.group_key();
                let operator_hiding = noist_ctx.hiding_group_nonce();
                let operator_binding = noist_ctx.post_binding_group_nonce();

                if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                    return false;
                }

                Some((active_dir_height, nonce_height, noist_ctx, musig_ctx))
            }
        };

        let zkp_contingent_ctxes = match self.zkp_contingent_nonces.len() {
            0 => None,
            _ => {
                // TODO:
                let zkp_contingent_message = [0xffu8; 32];

                let mut dkg_dir_ = active_dkg_dir.lock().await;

                let noist_ctx =
                    match dkg_dir_.pick_signing_session(zkp_contingent_message, None, true) {
                        Some(session) => session,
                        None => return false,
                    };

                let nonce_height = noist_ctx.nonce_height();

                let mut keys: Vec<Point> = self
                    .zkp_contingent_nonces
                    .iter()
                    .map(|(account, _)| account.key().clone())
                    .collect();

                keys.push(noist_ctx.group_key());

                let key_agg_ctx = match MusigKeyAggCtx::new(&keys, None) {
                    Some(ctx) => ctx,
                    None => return false,
                };

                let mut musig_ctx = match MusigSessionCtx::new(&key_agg_ctx, zkp_contingent_message)
                {
                    Some(ctx) => ctx,
                    None => return false,
                };

                for (msg_sender, (hiding, binding)) in self.zkp_contingent_nonces.iter() {
                    if !musig_ctx.insert_nonce(
                        msg_sender.key(),
                        hiding.to_owned(),
                        binding.to_owned(),
                    ) {
                        return false;
                    }
                }

                let operator_key = noist_ctx.group_key();
                let operator_hiding = noist_ctx.hiding_group_nonce();
                let operator_binding = noist_ctx.post_binding_group_nonce();

                if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                    return false;
                }

                Some((active_dir_height, nonce_height, noist_ctx, musig_ctx))
            }
        };

        // Lift prevtxos:

        let mut lift_prevtxo_ctxes = HashMap::<
            Account,
            HashMap<
                Lift,
                (
                    DKGDirHeight,
                    DKGNonceHeight,
                    NOISTSessionCtx,
                    MusigSessionCtx,
                ),
            >,
        >::new();

        for (msg_sender, lift_prevtxos) in self.lift_prevtxo_nonces.iter() {
            let mut ctxes = HashMap::<
                Lift,
                (
                    DKGDirHeight,
                    DKGNonceHeight,
                    NOISTSessionCtx,
                    MusigSessionCtx,
                ),
            >::new();

            for (lift, (hiding, binding)) in lift_prevtxos.iter() {
                let operator_key = lift.operator_key();

                let (dir_height, nonce_index, noist_ctx, musig_ctx) = {
                    // TODO:
                    let lift_prevtxo_message = [0xffu8; 32];

                    let dkg_dir: DKG_DIRECTORY = {
                        let dkg_manager_ = dkg_manager.lock().await;
                        match dkg_manager_.directory_by_key(operator_key).await {
                            Some(manager) => manager,
                            None => return false,
                        }
                    };

                    let dir_height = {
                        let dkg_dir_ = dkg_dir.lock().await;
                        dkg_dir_.dir_height()
                    };

                    let mut dkg_dir_ = dkg_dir.lock().await;

                    let noist_ctx =
                        match dkg_dir_.pick_signing_session(lift_prevtxo_message, None, true) {
                            Some(session) => session,
                            None => return false,
                        };

                    let nonce_height = noist_ctx.nonce_height();
                    let remote_key = msg_sender.key();

                    let key_agg_ctx = match lift.key_agg_ctx() {
                        Some(ctx) => ctx,
                        None => return false,
                    };

                    let mut musig_ctx =
                        match MusigSessionCtx::new(&key_agg_ctx, lift_prevtxo_message) {
                            Some(ctx) => ctx,
                            None => return false,
                        };

                    if !musig_ctx.insert_nonce(remote_key, hiding.to_owned(), binding.to_owned()) {
                        return false;
                    }

                    let operator_key = noist_ctx.group_key();
                    let operator_hiding = noist_ctx.hiding_group_nonce();
                    let operator_binding = noist_ctx.post_binding_group_nonce();

                    if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                        return false;
                    }

                    (dir_height, nonce_height, noist_ctx, musig_ctx)
                };
                if let Some(_) = ctxes.insert(
                    lift.to_owned(),
                    (dir_height, nonce_index, noist_ctx, musig_ctx),
                ) {
                    return false;
                }
            }

            if let Some(_) = lift_prevtxo_ctxes.insert(msg_sender.to_owned(), ctxes) {
                return false;
            }
        }

        // Connector TXOs:
        let mut connector_txo_ctxes = HashMap::<
            Account,
            Vec<(
                DKGDirHeight,
                DKGNonceHeight,
                NOISTSessionCtx,
                MusigSessionCtx,
            )>,
        >::new();

        for (msg_sender, connector_txos) in self.connector_txo_nonces.iter() {
            let mut ctxes = Vec::<(
                DKGDirHeight,
                DKGNonceHeight,
                NOISTSessionCtx,
                MusigSessionCtx,
            )>::new();

            for (hiding, binding) in connector_txos.iter() {
                let (dir_height, nonce_height, noist_ctx, musig_ctx) = {
                    // TODO:
                    let connector_txo_message = [0xffu8; 32];

                    let mut dkg_dir_ = active_dkg_dir.lock().await;

                    let noist_ctx =
                        match dkg_dir_.pick_signing_session(connector_txo_message, None, true) {
                            Some(session) => session,
                            None => return false,
                        };

                    let nonce_height = noist_ctx.nonce_height();
                    let remote_key = msg_sender.key();
                    let operator_key = noist_ctx.group_key();

                    let connector = Connector::new(remote_key, operator_key);

                    let key_agg_ctx = match connector.key_agg_ctx() {
                        Some(ctx) => ctx,
                        None => return false,
                    };

                    let mut musig_ctx =
                        match MusigSessionCtx::new(&key_agg_ctx, connector_txo_message) {
                            Some(ctx) => ctx,
                            None => return false,
                        };

                    if !musig_ctx.insert_nonce(remote_key, hiding.to_owned(), binding.to_owned()) {
                        return false;
                    }

                    let operator_key = noist_ctx.group_key();
                    let operator_hiding = noist_ctx.hiding_group_nonce();
                    let operator_binding = noist_ctx.post_binding_group_nonce();

                    if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                        return false;
                    }

                    (active_dir_height, nonce_height, noist_ctx, musig_ctx)
                };

                ctxes.push((dir_height, nonce_height, noist_ctx, musig_ctx));
            }

            if let Some(_) = connector_txo_ctxes.insert(msg_sender.to_owned(), ctxes) {
                return false;
            }
        }

        // Set values:
        self.payload_auth_ctxes = payload_auth_ctxes;
        self.vtxo_projector_ctxes = vtxo_projector_ctxes;
        self.connector_projector_ctxes = connector_projector_ctxes;
        self.zkp_contingent_ctxes = zkp_contingent_ctxes;
        self.lift_prevtxo_ctxes = lift_prevtxo_ctxes;
        self.connector_txo_ctxes = connector_txo_ctxes;

        true
    }

    /// Locks the session, preventing further `NSessionCommit`s from joining this session.  
    /// New `NSessionCommit`s are put on hold until the `CSessionStage` is set back to `on`.
    pub async fn lock(&mut self) -> bool {
        self.stage = CSessionStage::Locked;
        self.set_ctxes().await
    }

    /// Returns the respective `CSessionOpCov`s for the DKG operators.
    /// The coordinator does this immediately after locking the session (simultaneously with commitack).
    /// `CSessionOpCov`s contain the post-round-one MuSig contexts to be filled with individual partial signatures (just like `CSessionCommitAck`).
    pub fn opcov(&self) -> Option<CSessionOpCov> {
        // Allowed only in the locked stage
        if self.stage != CSessionStage::Locked {
            return None;
        }

        let msg_senders = self.msg_senders();
        let liftups = self.liftups.clone();
        let recharges = self.recharges.clone();
        let vanillas = self.vanillas.clone();
        let calls = self.calls.clone();
        let reserveds = self.reserveds.clone();

        let payload_auth_musig_ctx = match &self.payload_auth_ctxes {
            Some((dir_height, nonce_height, _, musig_ctx)) => (
                dir_height.to_owned(),
                nonce_height.to_owned(),
                musig_ctx.to_owned(),
            ),
            None => return None,
        };

        let vtxo_projector_musig_ctx = match &self.vtxo_projector_ctxes {
            Some((dir_height, nonce_height, _, musig_ctx)) => Some((
                dir_height.to_owned(),
                nonce_height.to_owned(),
                musig_ctx.to_owned(),
            )),
            None => return None,
        };

        let connector_projector_musig_ctx = match &self.connector_projector_ctxes {
            Some((dir_height, nonce_height, _, musig_ctx)) => Some((
                dir_height.to_owned(),
                nonce_height.to_owned(),
                musig_ctx.to_owned(),
            )),
            None => return None,
        };

        let zkp_contingent_musig_ctx = match &self.zkp_contingent_ctxes {
            Some((dir_height, nonce_height, _, musig_ctx)) => Some((
                dir_height.to_owned(),
                nonce_height.to_owned(),
                musig_ctx.to_owned(),
            )),
            None => return None,
        };

        let lift_prevtxo_ctxes_opcov = self
            .lift_prevtxo_ctxes
            .clone()
            .into_iter()
            .map(|(account, inner_map)| {
                let new_inner_map = inner_map
                    .into_iter()
                    .map(|(lift, (dir_height, nonce_height, _, musig_ctx))| {
                        (lift, (dir_height, nonce_height, musig_ctx))
                    })
                    .collect();
                (account, new_inner_map)
            })
            .collect();

        let connector_txo_musig_ctxes_opcov = self
            .connector_txo_ctxes
            .clone()
            .into_iter()
            .map(|(account, vec)| {
                let new_vec = vec
                    .into_iter()
                    .map(|(dir_height, nonce_height, _, musig_ctx)| {
                        (dir_height, nonce_height, musig_ctx)
                    })
                    .collect();
                (account, new_vec)
            })
            .collect();

        let opcov = CSessionOpCov::new(
            msg_senders,
            liftups,
            recharges,
            vanillas,
            calls,
            reserveds,
            payload_auth_musig_ctx,
            vtxo_projector_musig_ctx,
            connector_projector_musig_ctx,
            zkp_contingent_musig_ctx,
            lift_prevtxo_ctxes_opcov,
            connector_txo_musig_ctxes_opcov,
        );

        Some(opcov)
    }

    /// Returns the respective `CSessionCommitAck`s for accounts that have previously committed.  
    /// The coordinator does this immediately after locking the session.
    /// `CSessionCommitAck`s contain the post-round-one MuSig contexts to be filled with individual partial signatures.
    /// Those individual partial signatures are returned as part of the subsequent `NSessionUphold`s.
    pub fn commitack(&self, account: Account) -> Option<CSessionCommitAck> {
        // Allowed only in the locked stage
        if self.stage != CSessionStage::Locked {
            return None;
        }

        if !self.msg_senders.contains(&account) {
            return None;
        }

        let msg_senders = self.msg_senders();
        let liftups = self.liftups.clone();
        let recharges = self.recharges.clone();
        let vanillas = self.vanillas.clone();
        let calls = self.calls.clone();
        let reserveds = self.reserveds.clone();

        let payload_auth_musig_ctx = self.payload_auth_ctxes.clone()?.3;

        let vtxo_projector_musig_ctx = match &self.vtxo_projector_ctxes {
            Some((_, _, _, ctx)) => Some(ctx.to_owned()),
            None => return None,
        };

        let connector_projector_musig_ctx = match &self.connector_projector_ctxes {
            Some((_, _, _, ctx)) => Some(ctx.to_owned()),
            None => return None,
        };

        let zkp_contingent_musig_ctx = match &self.zkp_contingent_ctxes {
            Some((_, _, _, ctx)) => Some(ctx.to_owned()),
            None => return None,
        };

        let mut lift_prevtxo_musig_ctxes = HashMap::<Lift, MusigSessionCtx>::new();

        for (account, lift_map) in self.lift_prevtxo_ctxes.iter() {
            if account.key() == account.key() {
                for (lift, (_, _, _, ctx)) in lift_map.iter() {
                    lift_prevtxo_musig_ctxes.insert(lift.to_owned(), ctx.to_owned());
                }
            }
        }

        // TODO: prune connectors
        let mut connector_txo_musig_ctxes = Vec::<MusigSessionCtx>::new();

        for (account, ctxes) in self.connector_txo_ctxes.iter() {
            if account.key() == account.key() {
                for (_, _, _, ctx) in ctxes {
                    connector_txo_musig_ctxes.push(ctx.to_owned());
                }
            }
        }

        let commitack = CSessionCommitAck::new(
            account,
            msg_senders,
            liftups,
            recharges,
            vanillas,
            calls,
            reserveds,
            payload_auth_musig_ctx,
            vtxo_projector_musig_ctx,
            connector_projector_musig_ctx,
            zkp_contingent_musig_ctx,
            lift_prevtxo_musig_ctxes,
            connector_txo_musig_ctxes,
        );

        Some(commitack)
    }

    pub fn insert_opcovack(&mut self, opcovack: OSessionOpCovAck) -> bool {
        // Allowed only in the locked stage
        if self.stage != CSessionStage::Locked {
            return false;
        }

        let signatory = opcovack.signatory();

        // Payload auth
        if let Some((_, _, noist_ctx, _)) = &mut self.payload_auth_ctxes {
            if let Some(partial_sig) = opcovack.payload_auth_partial_sig() {
                if !noist_ctx.insert_partial_sig(signatory, partial_sig) {
                    return false;
                }
            }
        }

        // VTXO projector
        if let Some((_, _, noist_ctx, _)) = &mut self.vtxo_projector_ctxes {
            if let Some(partial_sig) = opcovack.vtxo_projector_partial_sig() {
                if !noist_ctx.insert_partial_sig(signatory, partial_sig) {
                    return false;
                }
            }
        }

        // Connector projector
        if let Some((_, _, noist_ctx, _)) = &mut self.connector_projector_ctxes {
            if let Some(partial_sig) = opcovack.connector_projector_partial_sig() {
                if !noist_ctx.insert_partial_sig(signatory, partial_sig) {
                    return false;
                }
            }
        }

        // zkp contingent
        if let Some((_, _, noist_ctx, _)) = &mut self.zkp_contingent_ctxes {
            if let Some(partial_sig) = opcovack.zkp_contingent_partial_sig() {
                if !noist_ctx.insert_partial_sig(signatory, partial_sig) {
                    return false;
                }
            }
        }

        // Lift prevtxo
        let opcovack_lift_prevtxo_partial_sigs = opcovack.lift_prevtxo_partial_sigs();
        for (account, ctxes) in self.lift_prevtxo_ctxes.iter_mut() {
            let account_partial_sigs = match opcovack_lift_prevtxo_partial_sigs.get(account) {
                Some(sigs) => sigs,
                None => return false,
            };

            for (lift, (_, _, noist_ctx, _)) in ctxes.iter_mut() {
                let partial_sig = match account_partial_sigs.get(lift) {
                    Some(sig) => sig,
                    None => return false,
                };

                if let Some(sig) = partial_sig {
                    if !noist_ctx.insert_partial_sig(opcovack.signatory(), sig.to_owned()) {
                        return false;
                    }
                }
            }
        }

        // Connector txo
        let opcovack_connector_txo_partial_sigs = opcovack.connector_txo_partial_sigs();
        for (account, ctxes) in self.connector_txo_ctxes.iter_mut() {
            let account_partial_sigs = match opcovack_connector_txo_partial_sigs.get(account) {
                Some(sigs) => sigs,
                None => return false,
            };

            for (index, (_, _, noist_ctx, _)) in ctxes.iter_mut().enumerate() {
                let partial_sig = match account_partial_sigs.get(index) {
                    Some(sig) => sig,
                    None => return false,
                };

                if let Some(sig) = partial_sig {
                    if !noist_ctx.insert_partial_sig(opcovack.signatory(), sig.to_owned()) {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Attempts to insert `NSessionUphold` into the session,
    /// which contains partial signatures for the MuSig contexts
    /// requested as part of the earlier `CSessionCommitAck`.
    pub fn insert_uphold(
        &mut self,
        auth_uphold: Authenticable<NSessionUphold>,
    ) -> Result<(), CSessionUpholdError> {
        if !auth_uphold.authenticate() {
            return Err(CSessionUpholdError::AuthErr);
        }

        let uphold = auth_uphold.object();

        let account_key = uphold.msg_sender().key();

        if auth_uphold.key() != account_key.serialize_xonly() {
            return Err(CSessionUpholdError::AuthErr);
        }

        // Insert payload auth partial sig
        if let Some((_, _, _, ctx)) = &mut self.payload_auth_ctxes {
            if !ctx.insert_partial_sig(account_key, uphold.payload_auth_partial_sig()) {
                return Err(CSessionUpholdError::InvalidPayloadAuthSig);
            }
        }

        // Insert VTXO projector partial sig
        if let Some((_, _, _, ctx)) = &mut self.vtxo_projector_ctxes {
            let vtxo_projector_partial_sig = match uphold.vtxo_projector_partial_sig() {
                Some(sig) => sig,
                None => return Err(CSessionUpholdError::MissingVTXOProjectorSig),
            };

            if !ctx.insert_partial_sig(account_key, vtxo_projector_partial_sig) {
                return Err(CSessionUpholdError::InvalidVTXOProjectorSig);
            }
        }

        // Insert connector projector partial sig
        if let Some((_, _, _, ctx)) = &mut self.connector_projector_ctxes {
            let connector_projector_partial_sig = match uphold.connector_projector_partial_sig() {
                Some(sig) => sig,
                None => return Err(CSessionUpholdError::MissingConnectorProjectorSig),
            };

            if !ctx.insert_partial_sig(account_key, connector_projector_partial_sig) {
                return Err(CSessionUpholdError::InvalidConnectorProjectorSig);
            }
        }

        // Insert ZKP contingent partial sig
        if let Some((_, _, _, ctx)) = &mut self.zkp_contingent_ctxes {
            let zkp_contingent_partial_sig = match uphold.zkp_contingent_partial_sig() {
                Some(sig) => sig,
                None => return Err(CSessionUpholdError::MissingZKPContigentSig),
            };

            if !ctx.insert_partial_sig(account_key, zkp_contingent_partial_sig) {
                return Err(CSessionUpholdError::InvalidZKPContigentSig);
            }
        }

        // Insert lift prevtxo partial sigs
        let uphold_lift_prevtxo_partial_sigs = uphold.lift_prevtxo_partial_sigs();
        if let Some(musig_ctxes) = self.lift_prevtxo_ctxes.get_mut(&uphold.msg_sender()) {
            for (lift, (_, _, _, musig_ctx)) in musig_ctxes.iter_mut() {
                let partial_sig = match (&uphold_lift_prevtxo_partial_sigs).get(lift) {
                    Some(sig) => sig,
                    None => return Err(CSessionUpholdError::MissingLiftSig),
                };

                if !musig_ctx.insert_partial_sig(account_key, partial_sig.to_owned()) {
                    return Err(CSessionUpholdError::InvalidLiftSig);
                }
            }
        }

        // Insert connector txo partial sigs
        let uphold_connector_txo_partial_sigs = uphold.connector_txo_partial_sigs();
        if let Some(musig_ctxes) = self.connector_txo_ctxes.get_mut(&uphold.msg_sender()) {
            for (index, (_, _, _, musig_ctx)) in musig_ctxes.iter_mut().enumerate() {
                let partial_sig = match (&uphold_connector_txo_partial_sigs).get(index) {
                    Some(sig) => sig,
                    None => return Err(CSessionUpholdError::MissingConnectorSig),
                };

                if !musig_ctx.insert_partial_sig(account_key, partial_sig.to_owned()) {
                    return Err(CSessionUpholdError::InvalidConnectorSig);
                }
            }
        }

        Ok(())
    }

    /// Returns a lift of accounts who have failed to uphold their commitments.
    pub fn blame_list(&self) -> Vec<Account> {
        let mut blame_list = Vec::<Account>::new();

        // Payload auth scanning
        if let Some((_, _, _, musig_ctx)) = &self.payload_auth_ctxes {
            let musig_blame_list = musig_ctx.blame_list();

            for account in self.msg_senders.iter() {
                if musig_blame_list.contains(&account.key()) {
                    if !blame_list.contains(account) {
                        blame_list.push(account.to_owned());
                    }
                }
            }
        }

        // VTXO projector scanning
        if let Some((_, _, _, musig_ctx)) = &self.vtxo_projector_ctxes {
            let musig_blame_list = musig_ctx.blame_list();

            for account in self.msg_senders.iter() {
                if musig_blame_list.contains(&account.key()) {
                    if !blame_list.contains(account) {
                        blame_list.push(account.to_owned());
                    }
                }
            }
        }

        // Connector projector scanning
        if let Some((_, _, _, musig_ctx)) = &self.connector_projector_ctxes {
            let musig_blame_list = musig_ctx.blame_list();

            for account in self.msg_senders.iter() {
                if musig_blame_list.contains(&account.key()) {
                    if !blame_list.contains(account) {
                        blame_list.push(account.to_owned());
                    }
                }
            }
        }

        // ZKP contingent scanning
        if let Some((_, _, _, musig_ctx)) = &self.zkp_contingent_ctxes {
            let musig_blame_list = musig_ctx.blame_list();

            for account in self.msg_senders.iter() {
                if musig_blame_list.contains(&account.key()) {
                    if !blame_list.contains(account) {
                        blame_list.push(account.to_owned());
                    }
                }
            }
        }

        // Lifts scanning
        for (account, musig_ctxes) in self.lift_prevtxo_ctxes.iter() {
            for (_, (_, _, _, musig_ctx)) in musig_ctxes {
                let musig_blame_list = musig_ctx.blame_list();

                if musig_blame_list.contains(&account.key()) {
                    if !blame_list.contains(account) {
                        blame_list.push(account.to_owned());
                    }
                }
            }
        }

        // Connectors scanning
        for (account, musig_ctxes) in self.connector_txo_ctxes.iter() {
            for (_, _, _, musig_ctx) in musig_ctxes {
                let musig_blame_list = musig_ctx.blame_list();

                if musig_blame_list.contains(&account.key()) {
                    if !blame_list.contains(account) {
                        blame_list.push(account.to_owned());
                    }
                }
            }
        }

        blame_list
    }

    // is it final? who is missing? blaming..

    pub fn upheld(&mut self) {
        self.stage = CSessionStage::Upheld;
    }

    pub fn finalized(&mut self) {
        self.stage = CSessionStage::Finalized;
    }

    pub fn off(&mut self) {
        self.stage = CSessionStage::Off;
    }

    pub fn reset(&mut self) {
        self.msg_senders = Vec::<Account>::new();
        self.liftups = Vec::<Liftup>::new();
        self.recharges = Vec::<Recharge>::new();
        self.vanillas = Vec::<Vanilla>::new();
        self.calls = Vec::<Call>::new();
        self.reserveds = Vec::<Reserved>::new();
        self.payload_auth_nonces = HashMap::<Account, (Point, Point)>::new();
        self.payload_auth_ctxes = None;
        self.vtxo_projector_nonces = HashMap::<Account, (Point, Point)>::new();
        self.vtxo_projector_ctxes = None;
        self.connector_projector_nonces = HashMap::<Account, (Point, Point)>::new();
        self.connector_projector_ctxes = None;
        self.zkp_contingent_nonces = HashMap::<Account, (Point, Point)>::new();
        self.zkp_contingent_ctxes = None;
        self.lift_prevtxo_nonces = HashMap::<Account, HashMap<Lift, (Point, Point)>>::new();
        self.lift_prevtxo_ctxes = HashMap::<
            Account,
            HashMap<
                Lift,
                (
                    DKGDirHeight,
                    DKGNonceHeight,
                    NOISTSessionCtx,
                    MusigSessionCtx,
                ),
            >,
        >::new();
        self.connector_txo_nonces = HashMap::<Account, Vec<(Point, Point)>>::new();
        self.connector_txo_ctxes = HashMap::<
            Account,
            Vec<(
                DKGDirHeight,
                DKGNonceHeight,
                NOISTSessionCtx,
                MusigSessionCtx,
            )>,
        >::new();
    }
}
