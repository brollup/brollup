use crate::constructive::{entity::account::Account, txo::lift::Lift};
use secp::{Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// `OSessionOpCovAck` is similar to `NSessionUphold`, but it is for returning
/// partial covenant signatures from the operators rather than individual msg.senders.
/// `OSessionOpCovAck` is returned by the operators to the coordinator upon receiving
/// `CSessionOpCov` from the coordinator.
#[derive(Clone, Serialize, Deserialize)]
pub struct OSessionOpCovAck {
    signatory: Point,
    // Payload auth partial sig
    payload_auth_partial_sig: Option<Scalar>,
    // VTXO projector partial sig
    vtxo_projector_partial_sig: Option<Scalar>,
    // Connector projector partial sig
    connector_projector_partial_sig: Option<Scalar>,
    // ZKP contingent partial sig
    zkp_contingent_partial_sig: Option<Scalar>,
    // Lift prevtxo partial sigs
    lift_prevtxo_partial_sigs: HashMap<Account, HashMap<Lift, Option<Scalar>>>,
    // Connector partial sigs
    connector_txo_partial_sigs: HashMap<Account, Vec<Option<Scalar>>>,
}

impl OSessionOpCovAck {
    pub fn new(
        signatory: Point,
        payload_auth_partial_sig: Option<Scalar>,
        vtxo_projector_partial_sig: Option<Scalar>,
        connector_projector_partial_sig: Option<Scalar>,
        zkp_contingent_partial_sig: Option<Scalar>,
        lift_prevtxo_partial_sigs: HashMap<Account, HashMap<Lift, Option<Scalar>>>,
        connector_txo_partial_sigs: HashMap<Account, Vec<Option<Scalar>>>,
    ) -> OSessionOpCovAck {
        OSessionOpCovAck {
            signatory,
            payload_auth_partial_sig,
            vtxo_projector_partial_sig,
            connector_projector_partial_sig,
            zkp_contingent_partial_sig,
            lift_prevtxo_partial_sigs,
            connector_txo_partial_sigs,
        }
    }

    pub fn signatory(&self) -> Point {
        self.signatory.clone()
    }

    pub fn payload_auth_partial_sig(&self) -> Option<Scalar> {
        self.payload_auth_partial_sig.clone()
    }

    pub fn vtxo_projector_partial_sig(&self) -> Option<Scalar> {
        self.vtxo_projector_partial_sig.clone()
    }

    pub fn connector_projector_partial_sig(&self) -> Option<Scalar> {
        self.connector_projector_partial_sig.clone()
    }

    pub fn zkp_contingent_partial_sig(&self) -> Option<Scalar> {
        self.zkp_contingent_partial_sig.clone()
    }

    pub fn lift_prevtxo_partial_sigs(&self) -> HashMap<Account, HashMap<Lift, Option<Scalar>>> {
        self.lift_prevtxo_partial_sigs.clone()
    }

    pub fn connector_txo_partial_sigs(&self) -> HashMap<Account, Vec<Option<Scalar>>> {
        self.connector_txo_partial_sigs.clone()
    }
}
