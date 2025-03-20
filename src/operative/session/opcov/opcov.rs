use super::opcovack::OSessionOpCovAck;
use crate::{
    entry::Entry, key::KeyHolder, musig::session::MusigSessionCtx, txo::lift::Lift,
    entity::account::Account, DKG_MANAGER,
};
use secp::Scalar;
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
    entries: Vec<Entry>,
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
    connector_txo_musig_ctxes:
        HashMap<Account, Vec<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>>,
}

impl CSessionOpCov {
    pub fn new(
        entries: Vec<Entry>,
        payload_auth_musig_ctx: (DKGDirHeight, DKGNonceHeight, MusigSessionCtx),
        vtxo_projector_musig_ctx: Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
        connector_projector_musig_ctx: Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
        zkp_contingent_musig_ctx: Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
        lift_prevtxo_musig_ctxes: HashMap<
            Account,
            HashMap<Lift, (DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
        >,
        connector_txo_musig_ctxes: HashMap<
            Account,
            Vec<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)>,
        >,
    ) -> CSessionOpCov {
        CSessionOpCov {
            entries,
            payload_auth_musig_ctx,
            vtxo_projector_musig_ctx,
            connector_projector_musig_ctx,
            zkp_contingent_musig_ctx,
            lift_prevtxo_musig_ctxes,
            connector_txo_musig_ctxes,
        }
    }

    pub async fn opcovack(
        &self,
        dkg_manager: &mut DKG_MANAGER,
        keys: &KeyHolder,
    ) -> Option<OSessionOpCovAck> {
        let signatory = keys.public_key();

        // Payload auth
        let payload_auth_partial_sig = {
            let (dkg_dir_height, dkg_nonce_height, musig_ctx) = self.payload_auth_musig_ctx();

            let mut _dkg_manager = dkg_manager.lock().await;
            match _dkg_manager
                .musig_nested_signing_session(dkg_dir_height, dkg_nonce_height, musig_ctx, true)
                .await
            {
                Ok(signing_session) => {
                    match signing_session.partial_sign(keys.secret_key().serialize()) {
                        Some(partial_sig) => Some(partial_sig),
                        None => return None,
                    }
                }
                Err(_) => None,
            }
        };

        // VTXO projector
        let vtxo_projector_partial_sig = match self.vtxo_projector_musig_ctx() {
            Some((dkg_dir_height, dkg_nonce_height, musig_ctx)) => {
                let mut _dkg_manager = dkg_manager.lock().await;
                match _dkg_manager
                    .musig_nested_signing_session(dkg_dir_height, dkg_nonce_height, musig_ctx, true)
                    .await
                {
                    Ok(signing_session) => {
                        match signing_session.partial_sign(keys.secret_key().serialize()) {
                            Some(partial_sig) => Some(partial_sig),
                            None => return None,
                        }
                    }
                    Err(_) => None,
                }
            }
            None => None,
        };

        // Connector projector
        let connector_projector_partial_sig = match self.connector_projector_musig_ctx() {
            Some((dkg_dir_height, dkg_nonce_height, musig_ctx)) => {
                let mut _dkg_manager = dkg_manager.lock().await;
                match _dkg_manager
                    .musig_nested_signing_session(dkg_dir_height, dkg_nonce_height, musig_ctx, true)
                    .await
                {
                    Ok(signing_session) => {
                        match signing_session.partial_sign(keys.secret_key().serialize()) {
                            Some(partial_sig) => Some(partial_sig),
                            None => return None,
                        }
                    }
                    Err(_) => None,
                }
            }
            None => None,
        };

        // ZKP contingent
        let zkp_contingent_partial_sig = match self.zkp_contingent_musig_ctx() {
            Some((dkg_dir_height, dkg_nonce_height, musig_ctx)) => {
                let mut _dkg_manager = dkg_manager.lock().await;
                match _dkg_manager
                    .musig_nested_signing_session(dkg_dir_height, dkg_nonce_height, musig_ctx, true)
                    .await
                {
                    Ok(signing_session) => {
                        match signing_session.partial_sign(keys.secret_key().serialize()) {
                            Some(partial_sig) => Some(partial_sig),
                            None => return None,
                        }
                    }
                    Err(_) => None,
                }
            }
            None => None,
        };

        // All lifts
        let mut lift_prevtxo_partial_sigs =
            HashMap::<Account, HashMap<Lift, Option<Scalar>>>::new();

        for (account, musig_ctxes) in self.lift_prevtxo_musig_ctxes.iter() {
            let mut lift_partial_sigs = HashMap::<Lift, Option<Scalar>>::new();

            for (lift, (dkg_dir_height, dkg_nonce_height, musig_ctx)) in musig_ctxes {
                let mut _dkg_manager = dkg_manager.lock().await;
                match _dkg_manager
                    .musig_nested_signing_session(
                        dkg_dir_height.to_owned(),
                        dkg_nonce_height.to_owned(),
                        musig_ctx.to_owned(),
                        true,
                    )
                    .await
                {
                    Ok(signing_session) => {
                        let partial_sig =
                            match signing_session.partial_sign(keys.secret_key().serialize()) {
                                Some(partial_sig) => partial_sig,
                                None => return None,
                            };

                        lift_partial_sigs.insert(lift.to_owned(), Some(partial_sig));
                    }
                    Err(_) => {
                        lift_partial_sigs.insert(lift.to_owned(), None);
                    }
                }
            }

            lift_prevtxo_partial_sigs.insert(account.to_owned(), lift_partial_sigs);
        }

        // All connectors
        let mut connector_txo_partial_sigs = HashMap::<Account, Vec<Option<Scalar>>>::new();

        for (account, musig_ctxes) in self.connector_txo_musig_ctxes.iter() {
            let mut connector_partial_sigs = Vec::<Option<Scalar>>::new();

            for (dkg_dir_height, dkg_nonce_height, musig_ctx) in musig_ctxes {
                let mut _dkg_manager = dkg_manager.lock().await;
                match _dkg_manager
                    .musig_nested_signing_session(
                        dkg_dir_height.to_owned(),
                        dkg_nonce_height.to_owned(),
                        musig_ctx.to_owned(),
                        true,
                    )
                    .await
                {
                    Ok(signing_session) => {
                        let partial_sig =
                            match signing_session.partial_sign(keys.secret_key().serialize()) {
                                Some(partial_sig) => partial_sig,
                                None => return None,
                            };

                        connector_partial_sigs.push(Some(partial_sig));
                    }
                    Err(_) => {
                        connector_partial_sigs.push(None);
                    }
                }
            }

            connector_txo_partial_sigs.insert(account.to_owned(), connector_partial_sigs);
        }

        let opcovack = OSessionOpCovAck::new(
            signatory,
            payload_auth_partial_sig,
            vtxo_projector_partial_sig,
            connector_projector_partial_sig,
            zkp_contingent_partial_sig,
            lift_prevtxo_partial_sigs,
            connector_txo_partial_sigs,
        );

        Some(opcovack)
    }

    pub fn entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }

    pub fn payload_auth_musig_ctx(&self) -> (DKGDirHeight, DKGNonceHeight, MusigSessionCtx) {
        self.payload_auth_musig_ctx.clone()
    }

    pub fn vtxo_projector_musig_ctx(
        &self,
    ) -> Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)> {
        self.vtxo_projector_musig_ctx.clone()
    }

    pub fn connector_projector_musig_ctx(
        &self,
    ) -> Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)> {
        self.connector_projector_musig_ctx.clone()
    }

    pub fn zkp_contingent_musig_ctx(
        &self,
    ) -> Option<(DKGDirHeight, DKGNonceHeight, MusigSessionCtx)> {
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
