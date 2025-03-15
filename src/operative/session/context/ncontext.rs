use super::{
    commitack::CSessionCommitAck, uphold::NSessionUphold, upholdack::CSessionUpholdAck,
    upholderr::NSessionUpholdError,
};
use crate::{
    entry::Entry,
    into::IntoScalar,
    key::KeyHolder,
    schnorr::{self, Authenticable},
    session::commit::NSessionCommit,
    txo::lift::Lift,
    valtype::account::Account,
};
use secp::{Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const CONNECTORS_EXTRA_IN: u8 = 10;

#[derive(Clone, Serialize, Deserialize)]
pub struct NSessionCtx {
    secret_key: Scalar,
    entry: Entry,
    // Payload auth nonces
    payload_auth_secret_nonces: (Scalar, Scalar),
    payload_auth_public_nonces: (Point, Point),
    // VTXO projector nonces
    vtxo_projector_secret_nonces: (Scalar, Scalar),
    vtxo_projector_public_nonces: (Point, Point),
    // Connector projector nonces
    connector_projector_secret_nonces: (Scalar, Scalar),
    connector_projector_public_nonces: (Point, Point),
    // Connector projector nonces
    zkp_contingent_secret_nonces: (Scalar, Scalar),
    zkp_contingent_public_nonces: (Point, Point),
    // Lift prevtxo nonces
    lift_prevtxo_secret_nonces: HashMap<Lift, (Scalar, Scalar)>,
    lift_prevtxo_public_nonces: HashMap<Lift, (Point, Point)>,
    // Connector txo nonces
    connector_txo_secret_nonces: Vec<(Scalar, Scalar)>,
    connector_txo_public_nonces: Vec<(Point, Point)>,
}

impl NSessionCtx {
    pub fn new(key_holder: KeyHolder, entry: Entry) -> Option<NSessionCtx> {
        let secret_key = key_holder.secret_key();
        let public_key = secret_key.base_point_mul();

        if entry.account().key() != public_key {
            return None;
        }

        let nonces = gen_nonce(&entry)?;

        let ctx = NSessionCtx {
            secret_key,
            entry,
            //
            // Payload auth nonces
            payload_auth_secret_nonces: nonces.0,
            payload_auth_public_nonces: nonces.1,
            // VTXO projector nonces
            vtxo_projector_secret_nonces: nonces.2,
            vtxo_projector_public_nonces: nonces.3,
            // Connector projector nonces
            connector_projector_secret_nonces: nonces.4,
            connector_projector_public_nonces: nonces.5,
            // Connector projector nonces
            zkp_contingent_secret_nonces: nonces.6,
            zkp_contingent_public_nonces: nonces.7,
            // Lift prevtxo nonces
            lift_prevtxo_secret_nonces: nonces.8,
            lift_prevtxo_public_nonces: nonces.9,
            // Connector txo nonces
            connector_txo_secret_nonces: nonces.10,
            connector_txo_public_nonces: nonces.11,
        };

        Some(ctx)
    }

    pub fn account(&self) -> Account {
        self.entry.account()
    }

    pub fn entry(&self) -> Entry {
        self.entry.clone()
    }

    // Returns the commitment
    pub fn commit(&self) -> Option<Authenticable<NSessionCommit>> {
        let commit = NSessionCommit::new(
            self.entry(),
            self.payload_auth_public_nonces.0,
            self.payload_auth_public_nonces.1,
            self.vtxo_projector_public_nonces.0,
            self.vtxo_projector_public_nonces.1,
            self.connector_projector_public_nonces.0,
            self.connector_projector_public_nonces.1,
            self.zkp_contingent_public_nonces.0,
            self.zkp_contingent_public_nonces.1,
            &self.lift_prevtxo_public_nonces,
            &self.connector_txo_public_nonces,
        );

        let auth_commit = Authenticable::new(commit, self.secret_key.serialize())?;

        Some(auth_commit)
    }

    fn validate_commitack(&self, commitack: &CSessionCommitAck) -> bool {
        // Validate msg.sender key
        if commitack.account().key() != self.entry.account().key() {
            return false;
        }

        // Validate payload auth msg
        if !commitack.validate_payload_auth_msg() {
            return false;
        }

        // TODO: additional validations..

        true
    }

    /// Returns the commitment uphold upon receiving `CSessionCommitAck`.
    pub fn uphold(
        &self,
        commitack: &CSessionCommitAck,
    ) -> Result<Authenticable<NSessionUphold>, NSessionUpholdError> {
        if !self.validate_commitack(commitack) {
            return Err(NSessionUpholdError::CommitAckAuthErr);
        }

        // Payload auth partial sig
        let payload_auth_partial_sig = {
            match commitack.payload_auth_musig_ctx().partial_sign(
                self.secret_key,
                self.payload_auth_secret_nonces.0,
                self.payload_auth_secret_nonces.1,
            ) {
                Some(sig) => sig,
                None => return Err(NSessionUpholdError::PayloadAuthPartialSignErr),
            }
        };

        // VTXO projector partial sig
        let vtxo_projector_partial_sig = {
            match commitack.vtxo_projector_musig_ctx() {
                Some(ctx) => {
                    match ctx.partial_sign(
                        self.secret_key,
                        self.vtxo_projector_secret_nonces.0,
                        self.vtxo_projector_secret_nonces.1,
                    ) {
                        Some(sig) => Some(sig),
                        None => return Err(NSessionUpholdError::VTXOProjectorPartialSignErr),
                    }
                }
                None => None,
            }
        };

        // Connector projector partial sig
        let connector_projector_partial_sig = {
            match commitack.connector_projector_musig_ctx() {
                Some(ctx) => {
                    match ctx.partial_sign(
                        self.secret_key,
                        self.connector_projector_secret_nonces.0,
                        self.connector_projector_secret_nonces.1,
                    ) {
                        Some(sig) => Some(sig),
                        None => return Err(NSessionUpholdError::ConnectorProjectorPartialSignErr),
                    }
                }
                None => None,
            }
        };

        // ZKP contingent partial sig
        let zkp_contingent_partial_sig = {
            match commitack.zkp_contingent_musig_ctx() {
                Some(ctx) => {
                    match ctx.partial_sign(
                        self.secret_key,
                        self.zkp_contingent_secret_nonces.0,
                        self.zkp_contingent_secret_nonces.1,
                    ) {
                        Some(sig) => Some(sig),
                        None => return Err(NSessionUpholdError::ZKPContigentPartialSignErr),
                    }
                }
                None => None,
            }
        };

        // Lift prevtxos partial sigs
        let mut lift_prevtxo_partial_sigs = HashMap::<Lift, Scalar>::new();

        for (lift, musig_ctx) in commitack.lift_prevtxo_musig_ctxes() {
            let (secret_hiding_nonce, secet_binding_nonce) = self
                .lift_prevtxo_secret_nonces
                .get(&lift)
                .ok_or(NSessionUpholdError::UnabletoFindLiftSecretNonces)?;

            let partial_sig = match musig_ctx.partial_sign(
                self.secret_key,
                secret_hiding_nonce.to_owned(),
                secet_binding_nonce.to_owned(),
            ) {
                Some(sig) => sig,
                None => return Err(NSessionUpholdError::LiftPrevtxoPartialSignErr),
            };

            lift_prevtxo_partial_sigs.insert(lift, partial_sig);
        }

        // Connector txos partial sigs
        let mut connector_txo_partial_sigs = Vec::<Scalar>::new();

        for (index, musig_ctx) in commitack.connector_txo_musig_ctxes().iter().enumerate() {
            let (secret_hiding_nonce, secet_binding_nonce) = self
                .connector_txo_secret_nonces
                .get(index)
                .ok_or(NSessionUpholdError::UnabletoFindConnectorSecretNonces)?;

            let partial_sig = match musig_ctx.partial_sign(
                self.secret_key,
                secret_hiding_nonce.to_owned(),
                secet_binding_nonce.to_owned(),
            ) {
                Some(sig) => sig,
                None => return Err(NSessionUpholdError::ConnectorPartialSignErr),
            };

            connector_txo_partial_sigs.push(partial_sig);
        }

        let uphold = NSessionUphold::new(
            self.account(),
            payload_auth_partial_sig,
            vtxo_projector_partial_sig,
            connector_projector_partial_sig,
            zkp_contingent_partial_sig,
            lift_prevtxo_partial_sigs,
            connector_txo_partial_sigs,
        );

        let auth_uphold = Authenticable::new(uphold, self.secret_key.serialize())
            .ok_or(NSessionUpholdError::AuthenticableErr)?;

        Ok(auth_uphold)
    }

    fn validate_upholdack(&self, upholdack: &CSessionUpholdAck) -> bool {
        if upholdack.msg_sender().key() != self.entry.account().key() {
            return false;
        }

        // TODO: additional validations..

        true
    }

    pub fn forfeit(&self, upholdack: &CSessionUpholdAck) -> Option<()> {
        if !self.validate_upholdack(upholdack) {
            return None;
        }

        // TODO: additional validations..

        None
    }
}

// TODO:
fn num_connectors(_entry: &Entry) -> u8 {
    3 as u8 + CONNECTORS_EXTRA_IN
}

fn gen_nonce_tuple() -> Option<((Scalar, Scalar), (Point, Point))> {
    let hiding_secret_nonce = schnorr::generate_secret().into_scalar().ok()?;
    let binding_secret_nonce = schnorr::generate_secret().into_scalar().ok()?;

    let hiding_public_nonce = hiding_secret_nonce.base_point_mul();
    let binding_public_nonce = binding_secret_nonce.base_point_mul();

    Some((
        (hiding_secret_nonce, binding_secret_nonce),
        (hiding_public_nonce, binding_public_nonce),
    ))
}

pub fn gen_nonce(
    entry: &Entry,
) -> Option<(
    (Scalar, Scalar),                // Payload auth secret nonces
    (Point, Point),                  // Payload auth public nonces
    (Scalar, Scalar),                // VTXO projector secret nonces
    (Point, Point),                  // VTXO projector public nonces
    (Scalar, Scalar),                // Connector projector secret nonces
    (Point, Point),                  // Connector projector public nonces
    (Scalar, Scalar),                // ZKP contingent secret nonces
    (Point, Point),                  // ZKP contingent public nonces
    HashMap<Lift, (Scalar, Scalar)>, // Lift prevtxo secret nonces
    HashMap<Lift, (Point, Point)>,   // Lift prevtxo public nonces
    Vec<(Scalar, Scalar)>,           // Connector txo secret nonces
    Vec<(Point, Point)>,             // Connector txo public nonces
)> {
    // Collect common nonces:
    let (payload_auth_secret_nonces, payload_auth_public_nonces) = gen_nonce_tuple()?;
    let (vtxo_projector_secret_nonces, vtxo_projector_public_nonces) = gen_nonce_tuple()?;
    let (connector_projector_secret_nonces, connector_projector_public_nonces) = gen_nonce_tuple()?;
    let (zkp_contingent_secret_nonces, zkp_contingent_public_nonces) = gen_nonce_tuple()?;

    // Collect lift nonces
    let mut lift_prevtxo_secret_nonces = HashMap::<Lift, (Scalar, Scalar)>::new();
    let mut lift_prevtxo_public_nonces = HashMap::<Lift, (Point, Point)>::new();

    if let Some(liftup) = &entry.liftup() {
        for lift in liftup.lifts().iter() {
            let (secret_nonces, public_nonces) = gen_nonce_tuple()?;

            lift_prevtxo_secret_nonces.insert(lift.to_owned(), secret_nonces);
            lift_prevtxo_public_nonces.insert(lift.to_owned(), public_nonces);
        }
    }

    // Collect connector nonces
    let mut connector_txo_secret_nonces = Vec::<(Scalar, Scalar)>::new();
    let mut connector_txo_public_nonces = Vec::<(Point, Point)>::new();

    let num_connectors = num_connectors(entry);

    for _ in 0..num_connectors {
        let (secret_nonces, public_nonces) = gen_nonce_tuple()?;
        connector_txo_secret_nonces.push(secret_nonces);
        connector_txo_public_nonces.push(public_nonces);
    }

    Some((
        payload_auth_secret_nonces,
        payload_auth_public_nonces,
        vtxo_projector_secret_nonces,
        vtxo_projector_public_nonces,
        connector_projector_secret_nonces,
        connector_projector_public_nonces,
        zkp_contingent_secret_nonces,
        zkp_contingent_public_nonces,
        lift_prevtxo_secret_nonces,
        lift_prevtxo_public_nonces,
        connector_txo_secret_nonces,
        connector_txo_public_nonces,
    ))
}
