use crate::{
    entry::Entry, musig::session::MusigSessionCtx, txo::lift::Lift, valtype::account::Account,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// `CSessionCommitAck` is returned by the coordinator to the msg.senders
/// upon receiving `NSessionCommit` if the commitment is successful.
/// Otherwise, the coordinator responds with `CSessionCommitNack`.
/// `CSessionCommitAck` contains the MuSig contexts in which the respective msg.sender is a co-signer.
#[derive(Clone, Serialize, Deserialize)]
pub struct CSessionCommitAck {
    // Msg sender
    msg_sender: Account,
    // Entries
    entries: Vec<Entry>,
    // Payload auth
    payload_auth_musig_ctx: MusigSessionCtx,
    // VTXO projector
    vtxo_projector_musig_ctx: Option<MusigSessionCtx>,
    // Connector projector
    connector_projector_musig_ctx: Option<MusigSessionCtx>,
    // ZKP contingent
    zkp_contingent_musig_ctx: Option<MusigSessionCtx>,
    // Lift txos related to this account
    lift_prevtxo_musig_ctxes: HashMap<Lift, MusigSessionCtx>,
    // Connectors related to this account
    connector_txo_musig_ctxes: Vec<MusigSessionCtx>,
}

impl CSessionCommitAck {
    pub fn new(
        msg_sender: Account,
        entries: Vec<Entry>,
        payload_auth_musig_ctx: MusigSessionCtx,
        vtxo_projector_musig_ctx: Option<MusigSessionCtx>,
        connector_projector_musig_ctx: Option<MusigSessionCtx>,
        zkp_contingent_musig_ctx: Option<MusigSessionCtx>,
        lift_prevtxo_musig_ctxes: HashMap<Lift, MusigSessionCtx>,
        connector_txo_musig_ctxes: Vec<MusigSessionCtx>,
    ) -> CSessionCommitAck {
        CSessionCommitAck {
            msg_sender,
            entries,
            payload_auth_musig_ctx,
            vtxo_projector_musig_ctx,
            connector_projector_musig_ctx,
            zkp_contingent_musig_ctx,
            lift_prevtxo_musig_ctxes,
            connector_txo_musig_ctxes,
        }
    }

    pub fn msg_sender(&self) -> Account {
        self.msg_sender.clone()
    }

    pub fn entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }

    pub fn payload_auth_musig_ctx(&self) -> MusigSessionCtx {
        self.payload_auth_musig_ctx.clone()
    }

    pub fn vtxo_projector_musig_ctx(&self) -> Option<MusigSessionCtx> {
        self.vtxo_projector_musig_ctx.clone()
    }

    pub fn connector_projector_musig_ctx(&self) -> Option<MusigSessionCtx> {
        self.connector_projector_musig_ctx.clone()
    }

    pub fn zkp_contingent_musig_ctx(&self) -> Option<MusigSessionCtx> {
        self.zkp_contingent_musig_ctx.clone()
    }

    pub fn lift_prevtxo_musig_ctxes(&self) -> HashMap<Lift, MusigSessionCtx> {
        self.lift_prevtxo_musig_ctxes.clone()
    }

    pub fn connector_txo_musig_ctxes(&self) -> Vec<MusigSessionCtx> {
        self.connector_txo_musig_ctxes.clone()
    }
}
