use crate::{txo::lift::Lift, valtype::account::Account};
use secp::Scalar;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// `OSessionOpCovAck` is similar to `NSessionUphold`, but it is for returning
/// partial covenant signatures from the operators rather than individual msg.senders.
/// `OSessionOpCovAck` is returned by the operators to the coordinator upon receiving
/// `CSessionOpCov` from the coordinator.
#[derive(Clone, Serialize, Deserialize)]
pub struct OSessionOpCovAck {
    // Payload auth partial sig
    payload_auth_partial_sig: Scalar,
    // VTXO projector partial sig
    vtxo_projector_partial_sig: Option<Scalar>,
    // Connector projector partial sig
    connector_projector_partial_sig: Option<Scalar>,
    // ZKP contingent partial sig
    zkp_contingent_partial_sig: Option<Scalar>,
    // Lift prevtxo partial sigs
    lift_prevtxo_partial_sigs: HashMap<Account, HashMap<Lift, Scalar>>,
    // Connector partial sigs
    connector_txo_partial_sigs: HashMap<Account, Vec<Scalar>>,
}

impl OSessionOpCovAck {
    pub fn new(
        payload_auth_partial_sig: Scalar,
        vtxo_projector_partial_sig: Option<Scalar>,
        connector_projector_partial_sig: Option<Scalar>,
        zkp_contingent_partial_sig: Option<Scalar>,
        lift_prevtxo_partial_sigs: HashMap<Account, HashMap<Lift, Scalar>>,
        connector_txo_partial_sigs: HashMap<Account, Vec<Scalar>>,
    ) -> OSessionOpCovAck {
        OSessionOpCovAck {
            payload_auth_partial_sig,
            vtxo_projector_partial_sig,
            connector_projector_partial_sig,
            zkp_contingent_partial_sig,
            lift_prevtxo_partial_sigs,
            connector_txo_partial_sigs,
        }
    }

    pub fn payload_auth_partial_sig(&self) -> Scalar {
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

    pub fn lift_prevtxo_partial_sigs(&self) -> HashMap<Account, HashMap<Lift, Scalar>> {
        self.lift_prevtxo_partial_sigs.clone()
    }

    pub fn connector_txo_partial_sigs(&self) -> HashMap<Account, Vec<Scalar>> {
        self.connector_txo_partial_sigs.clone()
    }
}
