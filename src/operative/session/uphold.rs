use crate::{txo::lift::Lift, valtype::account::Account};
use secp::Scalar;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct NSessionUphold {
    // Account
    account: Account,
    // Payload auth partial sig
    payload_auth_partial_sig: Scalar,
    // VTXO projector partial sig
    vtxo_projector_partial_sig: Option<Scalar>,
    // Connector projector partial sig
    connector_projector_partial_sig: Option<Scalar>,
    // ZKP contingent partial sig
    zkp_contingent_partial_sig: Option<Scalar>,
    // Lift prevtxo partial sigs
    lift_prevtxo_partial_sigs: HashMap<Lift, Scalar>,
    // Connector partial sigs
    connector_txo_partial_sigs: Vec<Scalar>,
}

impl NSessionUphold {
    pub fn new(
        account: Account,
        payload_auth_partial_sig: Scalar,
        vtxo_projector_partial_sig: Option<Scalar>,
        connector_projector_partial_sig: Option<Scalar>,
        zkp_contingent_partial_sig: Option<Scalar>,
        lift_prevtxo_partial_sigs: HashMap<Lift, Scalar>,
        connector_txo_partial_sigs: Vec<Scalar>,
    ) -> NSessionUphold {
        NSessionUphold {
            account,
            payload_auth_partial_sig,
            vtxo_projector_partial_sig,
            connector_projector_partial_sig,
            zkp_contingent_partial_sig,
            lift_prevtxo_partial_sigs,
            connector_txo_partial_sigs,
        }
    }

    pub fn account(&self) -> Account {
        self.account.clone()
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

    pub fn lift_prevtxo_partial_sigs(&self) -> HashMap<Lift, Scalar> {
        self.lift_prevtxo_partial_sigs.clone()
    }

    pub fn connector_txo_partial_sigs(&self) -> Vec<Scalar> {
        self.connector_txo_partial_sigs.clone()
    }
}
