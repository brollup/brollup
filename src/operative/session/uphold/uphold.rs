use crate::{
    constructive::{entity::account::Account, txo::lift::Lift},
    transmutive::{
        hash::{Hash, HashTag},
        secp::authenticable::AuthSighash,
    },
};
use secp::Scalar;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// `NSessionUphold` is a follow-up request from the msg.sender for the coordinator to uphold the session.
/// It is sent by the msg.senders to the coordinator, who then responds with `CSessionUpholdAck`.
/// `NSessionUphold` contains the covenant partial signatures and is returned by the msg.senders to the coordinator
/// upon receiving `CSessionCommitAck` from the coordinator.
#[derive(Clone, Serialize, Deserialize)]
pub struct NSessionUphold {
    // Account
    msg_sender: Account,
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
        msg_sender: Account,
        payload_auth_partial_sig: Scalar,
        vtxo_projector_partial_sig: Option<Scalar>,
        connector_projector_partial_sig: Option<Scalar>,
        zkp_contingent_partial_sig: Option<Scalar>,
        lift_prevtxo_partial_sigs: HashMap<Lift, Scalar>,
        connector_txo_partial_sigs: Vec<Scalar>,
    ) -> NSessionUphold {
        NSessionUphold {
            msg_sender,
            payload_auth_partial_sig,
            vtxo_projector_partial_sig,
            connector_projector_partial_sig,
            zkp_contingent_partial_sig,
            lift_prevtxo_partial_sigs,
            connector_txo_partial_sigs,
        }
    }

    pub fn msg_sender(&self) -> Account {
        self.msg_sender.clone()
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

impl AuthSighash for NSessionUphold {
    fn auth_sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        // Account
        preimage.extend(self.msg_sender.key().serialize_xonly());

        // payload_auth_partial_sig
        preimage.extend(self.payload_auth_partial_sig.serialize());

        // vtxo_projector_partial_sig
        match &self.vtxo_projector_partial_sig {
            Some(sig) => preimage.extend(sig.serialize()),
            None => preimage.push(0x00),
        };

        // connector_projector_partial_sig
        match &self.connector_projector_partial_sig {
            Some(sig) => preimage.extend(sig.serialize()),
            None => preimage.push(0x00),
        };

        // zkp_contingent_partial_sig
        match &self.zkp_contingent_partial_sig {
            Some(sig) => preimage.extend(sig.serialize()),
            None => preimage.push(0x00),
        };

        // Lifts
        let mut lift_prevtxos_sorted: Vec<(Lift, Scalar)> =
            self.lift_prevtxo_partial_sigs().into_iter().collect();
        lift_prevtxos_sorted.sort_by(|(lift_a, _), (lift_b, _)| lift_a.cmp(lift_b));

        for (lift, partial_sig) in lift_prevtxos_sorted.iter() {
            preimage.extend(lift.serialize());
            preimage.extend(partial_sig.serialize());
        }

        // Connectors
        for partial_sig in self.connector_txo_partial_sigs.iter() {
            preimage.extend(partial_sig.serialize());
        }

        preimage.hash(Some(HashTag::Sighash))
    }
}
