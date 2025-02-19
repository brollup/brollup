use crate::{
    entry::{call::Call, liftup::Liftup, recharge::Recharge, reserved::Reserved, vanilla::Vanilla},
    musig::{keyagg::MusigKeyAggCtx, session::MusigSessionCtx},
    registery::key_registery_index,
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CSessionStage {
    On,        // collect keys, hiding and binding nonces.
    Locked,    // no longer accepting remote. MusigNestingCtx ready.
    Finalized, // collected all partial sigs.
    Off,
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
    payload_auth_musig_ctx: Option<(u64, MusigSessionCtx)>,
    // VTXO projector:
    vtxo_projector_nonces: HashMap<Account, (Point, Point)>,
    vtxo_projector_musig_ctx: Option<(u64, MusigSessionCtx)>,
    // Connector projector:
    connector_projector_nonces: HashMap<Account, (Point, Point)>,
    connector_projector_musig_ctx: Option<(u64, MusigSessionCtx)>,
    // ZKP contingent:
    zkp_contingent_nonces: HashMap<Account, (Point, Point)>,
    zkp_contingent_musig_ctx: Option<(u64, MusigSessionCtx)>,
    // Lift txos:
    lift_prevtxo_nonces: HashMap<Account, HashMap<Lift, (Point, Point)>>,
    lift_prevtxo_musig_ctxes: HashMap<Account, HashMap<Lift, (u64, u64, MusigSessionCtx)>>,
    // Connectors:
    connector_txo_nonces: HashMap<Account, Vec<(Point, Point)>>,
    connector_txo_musig_ctxes: HashMap<Account, Vec<(u64, MusigSessionCtx)>>,
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
            payload_auth_musig_ctx: None,
            vtxo_projector_nonces: HashMap::<Account, (Point, Point)>::new(),
            vtxo_projector_musig_ctx: None,
            connector_projector_nonces: HashMap::<Account, (Point, Point)>::new(),
            connector_projector_musig_ctx: None,
            zkp_contingent_nonces: HashMap::<Account, (Point, Point)>::new(),
            zkp_contingent_musig_ctx: None,
            lift_prevtxo_nonces: HashMap::<Account, HashMap<Lift, (Point, Point)>>::new(),
            lift_prevtxo_musig_ctxes:
                HashMap::<Account, HashMap<Lift, (u64, u64, MusigSessionCtx)>>::new(),
            connector_txo_nonces: HashMap::<Account, Vec<(Point, Point)>>::new(),
            connector_txo_musig_ctxes: HashMap::<Account, Vec<(u64, MusigSessionCtx)>>::new(),
        };
        Arc::new(Mutex::new(session))
    }

    pub fn payload_auth_nonces(&self) -> HashMap<Account, (Point, Point)> {
        self.payload_auth_nonces.clone()
    }

    pub fn payload_auth_musig_ctx(&self) -> Option<(u64, MusigSessionCtx)> {
        self.payload_auth_musig_ctx.clone()
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

    pub fn on(&mut self) {
        self.reset();
        self.stage = CSessionStage::On;
    }

    async fn is_valid_commit(&self, commit: &NSessionCommit) -> bool {
        let dkg_manager: DKG_MANAGER = Arc::clone(&self.dkg_manager);

        let msg_sender = commit.account();
        let msg_sender_key = msg_sender.key();

        // #1 Overlap check
        if self.msg_senders.contains(&msg_sender) {
            return false;
        }

        // #2 Allowance check
        if allowance(msg_sender) {
            return false;
        }

        // #3 Lift prevtxouts validation
        for (lift, _) in commit.lift_prevtxo_nonces().iter() {
            // #1 Operator key validation
            {
                let lift_operator_key = lift.operator_key();
                let lift_remote_key = lift.remote_key();

                if lift_remote_key != msg_sender_key {
                    return false;
                }

                {
                    let dkg_manager_ = dkg_manager.lock().await;
                    if let None = dkg_manager_.directory_by_key(lift_operator_key).await {
                        return false;
                    }
                }
            }

            // #2 Outpoint validation
            {
                match lift.outpoint() {
                    Some(_outpoint) => {
                        // TODO: check if this is a valid outpoint.
                    }
                    None => return false,
                };
            }
        }

        // #4 TODO: Check for num of connectors:
        let _connector_count = self.connector_projector_nonces.len();

        true
    }

    pub async fn insert_commit(&mut self, auth_commit: &Authenticable<NSessionCommit>) -> bool {
        if !auth_commit.authenticate() {
            return false;
        }

        let commit = auth_commit.object();

        let mut msg_sender = commit.account();

        if auth_commit.key() != msg_sender.key().serialize_xonly() {
            return false;
        }

        // #1 Check commit validity.
        if !self.is_valid_commit(&commit).await {
            return false;
        };

        // #2 Registery info
        if let Some(registery_index) = key_registery_index(msg_sender.key()) {
            msg_sender.set_registery_index(registery_index);
        }

        // Insert into msg_senders
        self.msg_senders.push(msg_sender);

        // Insert liftup
        if let Some(liftup) = commit.liftup() {
            self.liftups.push(liftup);
        }

        // Insert recharge
        if let Some(recharge) = commit.recharge() {
            self.recharges.push(recharge);
        }

        // Insert vanilla
        if let Some(vanilla) = commit.vanilla() {
            self.vanillas.push(vanilla);
        }

        // Insert call
        if let Some(call) = commit.call() {
            self.calls.push(call);
        }

        // Insert reserved
        if let Some(reserved) = commit.reserved() {
            self.reserveds.push(reserved);
        }

        // Insert into payload_auth_nonces
        let payload_auth_nonces = commit.payload_auth_nonces();
        self.payload_auth_nonces
            .insert(msg_sender, payload_auth_nonces);

        // Insert into vtxo_projector_nonces
        let vtxo_projector_nonces = commit.vtxo_projector_nonces();
        self.vtxo_projector_nonces
            .insert(msg_sender, vtxo_projector_nonces);

        // Insert into connector_projector_nonces
        let connector_projector_nonces = commit.connector_projector_nonces();
        self.connector_projector_nonces
            .insert(msg_sender, connector_projector_nonces);

        // Insert into zkp_contingent_nonces
        let zkp_contingent_nonces = commit.zkp_contingent_nonces();
        self.zkp_contingent_nonces
            .insert(msg_sender, zkp_contingent_nonces);

        // Insert to lift nonces:
        let lift_prevtxo_nonces = commit.lift_prevtxo_nonces();
        self.lift_prevtxo_nonces
            .insert(msg_sender, lift_prevtxo_nonces);

        // Insert to connector nonces
        let connector_txo_nonces = commit.connector_txo_nonces();
        self.connector_txo_nonces
            .insert(msg_sender, connector_txo_nonces);

        true
    }

    // Sets the musig contexes upon collecting all commitments.
    async fn set_musig_ctxes(&mut self) -> bool {
        let dkg_manager: DKG_MANAGER = Arc::clone(&self.dkg_manager);

        let dkg_dir: DKG_DIRECTORY = {
            let dkg_manager_ = dkg_manager.lock().await;
            match dkg_manager_.active_directory() {
                Some(dir) => dir,
                None => return false,
            }
        };

        let payload_auth_musig_tuple = match self.payload_auth_nonces.len() {
            0 => return false,
            _ => {
                // TODO:
                let payload_auth_message = [0xffu8; 32];

                let mut dkg_dir_ = dkg_dir.lock().await;

                let noist_signing_session =
                    match dkg_dir_.pick_signing_session(payload_auth_message, None, false) {
                        Some(session) => session,
                        None => return false,
                    };

                let nonce_index = noist_signing_session.nonce_index();
                let mut keys: Vec<Point> = self
                    .payload_auth_nonces
                    .iter()
                    .map(|(account, _)| account.key().clone())
                    .collect();

                keys.push(noist_signing_session.group_key());

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

                let operator_key = noist_signing_session.group_key();
                let operator_hiding = noist_signing_session.hiding_group_nonce();
                let operator_binding = noist_signing_session.post_binding_group_nonce();

                if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                    return false;
                }

                Some((nonce_index, musig_ctx))
            }
        };

        let vtxo_projector_musig_tuple = match self.vtxo_projector_nonces.len() {
            0 => None,
            _ => {
                // TODO:
                let vtxo_projector_message = [0xffu8; 32];

                let mut dkg_dir_ = dkg_dir.lock().await;

                let noist_signing_session =
                    match dkg_dir_.pick_signing_session(vtxo_projector_message, None, false) {
                        Some(session) => session,
                        None => return false,
                    };

                let nonce_index = noist_signing_session.nonce_index();

                let remote_keys: Vec<Point> = self
                    .vtxo_projector_nonces
                    .iter()
                    .map(|(account, _)| account.key().clone())
                    .collect();

                let operator_key = noist_signing_session.group_key();

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

                let operator_key = noist_signing_session.group_key();
                let operator_hiding = noist_signing_session.hiding_group_nonce();
                let operator_binding = noist_signing_session.post_binding_group_nonce();

                if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                    return false;
                }

                Some((nonce_index, musig_ctx))
            }
        };

        let connector_projector_musig_tuple = match self.connector_projector_nonces.len() {
            0 => None,
            _ => {
                // TODO:
                let connector_projector_message = [0xffu8; 32];

                let mut dkg_dir_ = dkg_dir.lock().await;

                let noist_signing_session =
                    match dkg_dir_.pick_signing_session(connector_projector_message, None, false) {
                        Some(session) => session,
                        None => return false,
                    };

                let nonce_index = noist_signing_session.nonce_index();
                let remote_keys: Vec<Point> = self
                    .connector_projector_nonces
                    .iter()
                    .map(|(account, _)| account.key().clone())
                    .collect();

                let operator_key = noist_signing_session.group_key();

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

                let operator_key = noist_signing_session.group_key();
                let operator_hiding = noist_signing_session.hiding_group_nonce();
                let operator_binding = noist_signing_session.post_binding_group_nonce();

                if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                    return false;
                }

                Some((nonce_index, musig_ctx))
            }
        };

        let zkp_contingent_musig_tuple = match self.zkp_contingent_nonces.len() {
            0 => None,
            _ => {
                // TODO:
                let zkp_contingent_message = [0xffu8; 32];

                let mut dkg_dir_ = dkg_dir.lock().await;

                let noist_signing_session =
                    match dkg_dir_.pick_signing_session(zkp_contingent_message, None, false) {
                        Some(session) => session,
                        None => return false,
                    };

                let nonce_index = noist_signing_session.nonce_index();

                let mut keys: Vec<Point> = self
                    .zkp_contingent_nonces
                    .iter()
                    .map(|(account, _)| account.key().clone())
                    .collect();

                keys.push(noist_signing_session.group_key());

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

                let operator_key = noist_signing_session.group_key();
                let operator_hiding = noist_signing_session.hiding_group_nonce();
                let operator_binding = noist_signing_session.post_binding_group_nonce();

                if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                    return false;
                }

                Some((nonce_index, musig_ctx))
            }
        };

        // Lift prevtxos:

        let mut lift_prevtxo_musig_ctxes =
            HashMap::<Account, HashMap<Lift, (u64, u64, MusigSessionCtx)>>::new();

        for (msg_sender, lift_prevtxos) in self.lift_prevtxo_nonces.iter() {
            let mut musig_ctxes = HashMap::<Lift, (u64, u64, MusigSessionCtx)>::new();

            for (lift, (hiding, binding)) in lift_prevtxos.iter() {
                let operator_key = lift.operator_key();

                let (dir_height, nonce_index, musig_ctx) = {
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

                    let noist_signing_session =
                        match dkg_dir_.pick_signing_session(lift_prevtxo_message, None, false) {
                            Some(session) => session,
                            None => return false,
                        };

                    let nonce_index = noist_signing_session.nonce_index();
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

                    let operator_key = noist_signing_session.group_key();
                    let operator_hiding = noist_signing_session.hiding_group_nonce();
                    let operator_binding = noist_signing_session.post_binding_group_nonce();

                    if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                        return false;
                    }

                    (dir_height, nonce_index, musig_ctx)
                };
                if let Some(_) =
                    musig_ctxes.insert(lift.to_owned(), (dir_height, nonce_index, musig_ctx))
                {
                    return false;
                }
            }

            if let Some(_) = lift_prevtxo_musig_ctxes.insert(msg_sender.to_owned(), musig_ctxes) {
                return false;
            }
        }

        // Connector TXOs:

        let mut connector_txo_musig_ctxes = HashMap::<Account, Vec<(u64, MusigSessionCtx)>>::new();

        for (msg_sender, connector_txos) in self.connector_txo_nonces.iter() {
            let mut musig_ctxes = Vec::<(u64, MusigSessionCtx)>::new();

            for (hiding, binding) in connector_txos.iter() {
                let (nonce_index, musig_ctx) = {
                    // TODO:
                    let connector_txo_message = [0xffu8; 32];

                    let mut dkg_dir_ = dkg_dir.lock().await;

                    let noist_signing_session =
                        match dkg_dir_.pick_signing_session(connector_txo_message, None, false) {
                            Some(session) => session,
                            None => return false,
                        };

                    let nonce_index = noist_signing_session.nonce_index();
                    let remote_key = msg_sender.key();
                    let operator_key = noist_signing_session.group_key();

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

                    let operator_key = noist_signing_session.group_key();
                    let operator_hiding = noist_signing_session.hiding_group_nonce();
                    let operator_binding = noist_signing_session.post_binding_group_nonce();

                    if !musig_ctx.insert_nonce(operator_key, operator_hiding, operator_binding) {
                        return false;
                    }

                    (nonce_index, musig_ctx)
                };

                musig_ctxes.push((nonce_index, musig_ctx));
            }

            if let Some(_) = connector_txo_musig_ctxes.insert(msg_sender.to_owned(), musig_ctxes) {
                return false;
            }
        }

        // Set values:
        self.payload_auth_musig_ctx = payload_auth_musig_tuple;
        self.vtxo_projector_musig_ctx = vtxo_projector_musig_tuple;
        self.connector_projector_musig_ctx = connector_projector_musig_tuple;
        self.zkp_contingent_musig_ctx = zkp_contingent_musig_tuple;
        self.lift_prevtxo_musig_ctxes = lift_prevtxo_musig_ctxes;
        self.connector_txo_musig_ctxes = connector_txo_musig_ctxes;

        true
    }

    // Locks the session. No new commitments are allowed.
    pub async fn lock(&mut self) -> bool {
        self.stage = CSessionStage::Locked;
        self.set_musig_ctxes().await
    }

    // Returns the CommitAck for the respective account who have commited.
    pub fn into_commitack(&self, account: Account) -> Option<CSessionCommitAck> {
        if !self.msg_senders.contains(&account) {
            return None;
        }

        let msg_senders = self.msg_senders();
        let liftups = self.liftups.clone();
        let recharges = self.recharges.clone();
        let vanillas = self.vanillas.clone();
        let calls = self.calls.clone();
        let reserveds = self.reserveds.clone();

        let payload_auth_musig_ctx = self.payload_auth_musig_ctx()?.1;

        let vtxo_projector_musig_ctx = match &self.vtxo_projector_musig_ctx {
            Some((_, ctx)) => Some(ctx.to_owned()),
            None => return None,
        };

        let connector_projector_musig_ctx = match &self.connector_projector_musig_ctx {
            Some((_, ctx)) => Some(ctx.to_owned()),
            None => return None,
        };

        let zkp_contingent_musig_ctx = match &self.zkp_contingent_musig_ctx {
            Some((_, ctx)) => Some(ctx.to_owned()),
            None => return None,
        };

        let mut lift_prevtxo_musig_ctxes = HashMap::<Lift, MusigSessionCtx>::new();

        for (account, lift_map) in self.lift_prevtxo_musig_ctxes.iter() {
            if account.key() == account.key() {
                for (lift, (_, _, ctx)) in lift_map.iter() {
                    lift_prevtxo_musig_ctxes.insert(lift.to_owned(), ctx.to_owned());
                }
            }
        }

        let mut connector_txo_musig_ctxes = Vec::<MusigSessionCtx>::new();

        for (account, ctxes) in self.connector_txo_musig_ctxes.iter() {
            if account.key() == account.key() {
                for (_, ctx) in ctxes {
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
        self.payload_auth_musig_ctx = None;
        self.vtxo_projector_nonces = HashMap::<Account, (Point, Point)>::new();
        self.vtxo_projector_musig_ctx = None;
        self.connector_projector_nonces = HashMap::<Account, (Point, Point)>::new();
        self.connector_projector_musig_ctx = None;
        self.zkp_contingent_nonces = HashMap::<Account, (Point, Point)>::new();
        self.zkp_contingent_musig_ctx = None;
        self.lift_prevtxo_nonces = HashMap::<Account, HashMap<Lift, (Point, Point)>>::new();
        self.lift_prevtxo_musig_ctxes =
            HashMap::<Account, HashMap<Lift, (u64, u64, MusigSessionCtx)>>::new();
        self.connector_txo_nonces = HashMap::<Account, Vec<(Point, Point)>>::new();
        self.connector_txo_musig_ctxes = HashMap::<Account, Vec<(u64, MusigSessionCtx)>>::new();
    }
}
