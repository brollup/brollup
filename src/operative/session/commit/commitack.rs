use crate::{
    constructive::{entity::account::account::Account, entry::entry::Entry, txo::lift::Lift},
    transmutative::{
        hash::{Hash, HashTag},
        musig::session::MusigSessionCtx,
        secp::authenticable::AuthSighash,
    },
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
    account: Account,
    session_id: [u8; 32],
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
        account: Account,
        session_id: [u8; 32],
        entries: Vec<Entry>,
        payload_auth_musig_ctx: MusigSessionCtx,
        vtxo_projector_musig_ctx: Option<MusigSessionCtx>,
        connector_projector_musig_ctx: Option<MusigSessionCtx>,
        zkp_contingent_musig_ctx: Option<MusigSessionCtx>,
        lift_prevtxo_musig_ctxes: HashMap<Lift, MusigSessionCtx>,
        connector_txo_musig_ctxes: Vec<MusigSessionCtx>,
    ) -> CSessionCommitAck {
        CSessionCommitAck {
            account,
            session_id,
            entries,
            payload_auth_musig_ctx,
            vtxo_projector_musig_ctx,
            connector_projector_musig_ctx,
            zkp_contingent_musig_ctx,
            lift_prevtxo_musig_ctxes,
            connector_txo_musig_ctxes,
        }
    }

    pub fn account(&self) -> Account {
        self.account.clone()
    }

    pub fn session_id(&self) -> [u8; 32] {
        self.session_id
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

    fn payload_auth_msg(&self) -> [u8; 32] {
        let mut preimage = Vec::<u8>::new();

        // Session ID
        preimage.extend(self.session_id);

        // Entries
        for (index, entry) in self.entries.iter().enumerate() {
            let entry_sighash = entry.auth_sighash();
            preimage.extend((index as u32).to_le_bytes());
            preimage.extend(entry_sighash);
        }

        preimage.hash(Some(HashTag::PayloadAuth))
    }

    pub fn validate_payload_auth_msg(&self) -> bool {
        self.payload_auth_musig_ctx.message() == self.payload_auth_msg()
    }
}
