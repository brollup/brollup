use crate::entry::Entry;
use crate::hash::{Hash, HashTag};
use crate::schnorr::Sighash;
use crate::txo::lift::Lift;
use crate::entity::account::Account;
use secp::Point;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// `NSessionCommit` is a request from the msg.sender for the coordinator to commit to a session.
/// It is sent by the msg.senders to the coordinator, who then responds with `CSessionCommitAck`.
#[derive(Clone, Serialize, Deserialize)]
pub struct NSessionCommit {
    // Entry
    entry: Entry,
    // Payload auth nonces (hiding & binding)
    payload_auth_nonces: (Point, Point),
    // VTXO projector nonces (hiding & binding)
    vtxo_projector_nonces: (Point, Point),
    // Connector projector nonces (hiding & binding)
    connector_projector_nonces: (Point, Point),
    // ZKP contingent nonces (hiding & binding)
    zkp_contingent_nonces: (Point, Point),
    // Lift prevtxo nonces (Lift -> hiding & binding)
    lift_prevtxo_nonces: HashMap<Lift, (Point, Point)>,
    // Connector txo nonces (hiding & binding)
    connector_txo_nonces: Vec<(Point, Point)>,
}

impl NSessionCommit {
    pub fn new(
        entry: Entry,
        // Payload auth nonces
        payload_auth_hiding_nonce: Point,
        payload_auth_binding_nonce: Point,
        // VTXO projector nonces
        vtxo_projector_hiding_nonce: Point,
        vtxo_projector_binding_nonce: Point,
        // Connector projector nonces
        connector_projector_hiding_nonce: Point,
        connector_projector_binding_nonce: Point,
        // ZKP contingent nonces
        zkp_contingent_hiding_nonce: Point,
        zkp_contingent_binding_nonce: Point,
        // Lift prevtxo nonces
        lift_prevtxo_nonces: &HashMap<Lift, (Point, Point)>,
        connector_txo_nonces: &Vec<(Point, Point)>,
    ) -> NSessionCommit {
        NSessionCommit {
            entry,
            payload_auth_nonces: (payload_auth_hiding_nonce, payload_auth_binding_nonce),
            vtxo_projector_nonces: (vtxo_projector_hiding_nonce, vtxo_projector_binding_nonce),
            connector_projector_nonces: (
                connector_projector_hiding_nonce,
                connector_projector_binding_nonce,
            ),
            zkp_contingent_nonces: (zkp_contingent_hiding_nonce, zkp_contingent_binding_nonce),
            lift_prevtxo_nonces: lift_prevtxo_nonces.to_owned(),
            connector_txo_nonces: connector_txo_nonces.to_owned(),
        }
    }

    pub fn account(&self) -> Account {
        self.entry.account()
    }

    pub fn entry(&self) -> Entry {
        self.entry.clone()
    }

    pub fn payload_auth_nonces(&self) -> (Point, Point) {
        self.payload_auth_nonces.clone()
    }

    pub fn vtxo_projector_nonces(&self) -> (Point, Point) {
        self.vtxo_projector_nonces.clone()
    }

    pub fn connector_projector_nonces(&self) -> (Point, Point) {
        self.connector_projector_nonces.clone()
    }

    pub fn zkp_contingent_nonces(&self) -> (Point, Point) {
        self.zkp_contingent_nonces.clone()
    }

    pub fn lift_prevtxo_nonces(&self) -> HashMap<Lift, (Point, Point)> {
        self.lift_prevtxo_nonces.clone()
    }

    pub fn connector_txo_nonces(&self) -> Vec<(Point, Point)> {
        self.connector_txo_nonces.clone()
    }
}

impl Sighash for NSessionCommit {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        // Entry
        preimage.extend(self.entry.sighash());

        // Payload auth nonces
        preimage.extend(self.payload_auth_nonces.0.serialize());
        preimage.extend(self.payload_auth_nonces.1.serialize());

        // VTXO projector nonces
        preimage.extend(self.vtxo_projector_nonces.0.serialize());
        preimage.extend(self.vtxo_projector_nonces.1.serialize());

        // Connector projector nonces
        preimage.extend(self.connector_projector_nonces.0.serialize());
        preimage.extend(self.connector_projector_nonces.1.serialize());

        // ZKP contingent nonces
        preimage.extend(self.zkp_contingent_nonces.0.serialize());
        preimage.extend(self.zkp_contingent_nonces.1.serialize());

        // Lift prevtxo nonces
        for (lift, (hiding, binding)) in self.lift_prevtxo_nonces.iter() {
            preimage.extend(lift.serialize());
            preimage.extend(hiding.serialize());
            preimage.extend(binding.serialize());
        }

        // Connector txo nonces
        for (hiding, binding) in self.connector_txo_nonces.iter() {
            preimage.extend(hiding.serialize());
            preimage.extend(binding.serialize());
        }

        preimage.hash(Some(HashTag::Sighash))
    }
}
