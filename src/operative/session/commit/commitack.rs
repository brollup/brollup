use crate::{
    entry::{call::Call, liftup::Liftup, recharge::Recharge, reserved::Reserved, vanilla::Vanilla},
    musig::session::MusigSessionCtx,
    txo::lift::Lift,
    valtype::account::Account,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// `CSessionCommitAck` is returned by the coordinator to the msg.senders
/// upon receiving `NSessionCommit` if the commitment is successful.
/// Otherwise, the coordinator responds with `CSessionCommitError`.
#[derive(Clone, Serialize, Deserialize)]
pub struct CSessionCommitAck {
    // Account
    account: Account,
    // msg.senders
    msg_senders: Vec<Account>,
    // Liftups
    liftups: Vec<Liftup>,
    // Recharges
    recharges: Vec<Recharge>,
    // Vanillas
    vanillas: Vec<Vanilla>,
    // Calls
    calls: Vec<Call>,
    // Reserveds
    reserveds: Vec<Reserved>,
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
        account: Account,
        msg_senders: Vec<Account>,
        liftups: Vec<Liftup>,
        recharges: Vec<Recharge>,
        vanillas: Vec<Vanilla>,
        calls: Vec<Call>,
        reserveds: Vec<Reserved>,
        payload_auth_musig_ctx: MusigSessionCtx,
        vtxo_projector_musig_ctx: Option<MusigSessionCtx>,
        connector_projector_musig_ctx: Option<MusigSessionCtx>,
        zkp_contingent_musig_ctx: Option<MusigSessionCtx>,
        lift_prevtxo_musig_ctxes: HashMap<Lift, MusigSessionCtx>,
        connector_txo_musig_ctxes: Vec<MusigSessionCtx>,
    ) -> CSessionCommitAck {
        CSessionCommitAck {
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
        }
    }

    pub fn account(&self) -> Account {
        self.account
    }

    pub fn msg_senders(&self) -> Vec<Account> {
        self.msg_senders.clone()
    }

    pub fn liftups(&self) -> Vec<Liftup> {
        self.liftups.clone()
    }

    pub fn recharges(&self) -> Vec<Recharge> {
        self.recharges.clone()
    }

    pub fn vanillas(&self) -> Vec<Vanilla> {
        self.vanillas.clone()
    }

    pub fn calls(&self) -> Vec<Call> {
        self.calls.clone()
    }

    pub fn reserveds(&self) -> Vec<Reserved> {
        self.reserveds.clone()
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
