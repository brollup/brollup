use crate::{
    entry::{call::Call, liftup::Liftup, recharge::Recharge, reserved::Reserved, vanilla::Vanilla},
    musig::session::MusigSessionCtx,
    txo::lift::Lift,
    valtype::account::Account,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type DKGDirHeight = u64;
type DKGNonceHeight = u64;

/// `CSessionOpCov` is similar to `CSessionCommitAck`, but it is used for requesting
/// partial covenant signatures from the operators rather than individual msg.senders.
/// Therefore, `CSessionOpCov` contains all the MuSig contexts in which the DKG quorum is a co-signer.
#[derive(Clone, Serialize, Deserialize)]
pub struct CSessionOpCov {
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
    payload_auth_musig_ctx: (DKGDirHeight, DKGNonceHeight, MusigSessionCtx),
    // VTXO projector
    vtxo_projector_musig_ctx: Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
    // Connector projector
    connector_projector_musig_ctx: Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
    // ZKP contingent
    zkp_contingent_musig_ctx: Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
    // All lift txos
    lift_prevtxo_musig_ctxes:
        HashMap<Account, HashMap<Lift, (DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>>,
    // All connectors
    connector_txo_musig_ctxes: HashMap<Account, Vec<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>>,
}

impl CSessionOpCov {
    pub fn new(
        msg_senders: Vec<Account>,
        liftups: Vec<Liftup>,
        recharges: Vec<Recharge>,
        vanillas: Vec<Vanilla>,
        calls: Vec<Call>,
        reserveds: Vec<Reserved>,
        payload_auth_musig_ctx: (DKGDirHeight, DKGNonceHeight, MusigSessionCtx),
        vtxo_projector_musig_ctx: Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
        connector_projector_musig_ctx: Option<(DKGDirHeight,DKGNonceHeight, MusigSessionCtx)>,
        zkp_contingent_musig_ctx: Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
        lift_prevtxo_musig_ctxes: HashMap<
            Account,
            HashMap<Lift, (DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
        >,
        connector_txo_musig_ctxes: HashMap<Account, Vec<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>>,
    ) -> CSessionOpCov {
        CSessionOpCov {
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

    pub fn payload_auth_musig_ctx(&self) -> (DKGDirHeight, DKGNonceHeight, MusigSessionCtx) {
        self.payload_auth_musig_ctx.clone()
    }

    pub fn vtxo_projector_musig_ctx(&self) -> Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)> {
        self.vtxo_projector_musig_ctx.clone()
    }

    pub fn connector_projector_musig_ctx(&self) -> Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)> {
        self.connector_projector_musig_ctx.clone()
    }

    pub fn zkp_contingent_musig_ctx(&self) -> Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)> {
        self.zkp_contingent_musig_ctx.clone()
    }

    pub fn lift_prevtxo_musig_ctxes(
        &self,
    ) -> HashMap<Account, HashMap<Lift, (DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>> {
        self.lift_prevtxo_musig_ctxes.clone()
    }

    pub fn connector_txo_musig_ctxes(
        &self,
    ) -> HashMap<Account, Vec<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>> {
        self.connector_txo_musig_ctxes.clone()
    }
}
