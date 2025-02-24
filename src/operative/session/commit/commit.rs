use crate::hash::{Hash, HashTag};
use crate::schnorr::Sighash;
use crate::txo::lift::Lift;
use crate::{
    entry::{call::Call, liftup::Liftup, recharge::Recharge, reserved::Reserved, vanilla::Vanilla},
    valtype::account::Account,
};
use secp::Point;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// `NSessionCommit` is a request from the msg.sender for the coordinator to commit to a session. 
/// It is sent by the msg.senders to the coordinator, who then responds with `CSessionCommitAck`.
#[derive(Clone, Serialize, Deserialize)]
pub struct NSessionCommit {
    // Account
    account: Account,
    // Entries
    liftup: Option<Liftup>,
    recharge: Option<Recharge>,
    vanilla: Option<Vanilla>,
    call: Option<Call>,
    reserved: Option<Reserved>,
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
        account: Account,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
        vanilla: Option<Vanilla>,
        call: Option<Call>,
        reserved: Option<Reserved>,
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
            account,
            liftup,
            recharge,
            vanilla,
            call,
            reserved,
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
        self.account.clone()
    }

    pub fn liftup(&self) -> Option<Liftup> {
        self.liftup.clone()
    }

    pub fn recharge(&self) -> Option<Recharge> {
        self.recharge.clone()
    }

    pub fn vanilla(&self) -> Option<Vanilla> {
        self.vanilla.clone()
    }

    pub fn call(&self) -> Option<Call> {
        self.call.clone()
    }

    pub fn reserved(&self) -> Option<Reserved> {
        self.reserved.clone()
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

        // Account
        preimage.extend(self.account.key().serialize_xonly());

        // Liftup
        match self.liftup() {
            Some(liftup) => preimage.extend(liftup.serialize()),
            None => preimage.push(0x00),
        };

        // Recharge
        match self.recharge() {
            Some(recharge) => preimage.extend(recharge.serialize()),
            None => preimage.push(0x00),
        };

        // Vanilla
        match self.vanilla() {
            Some(vanilla) => preimage.extend(vanilla.serialize()),
            None => preimage.push(0x00),
        };

        // Call
        match self.call() {
            Some(call) => preimage.extend(call.serialize()),
            None => preimage.push(0x00),
        };

        // Reserved
        match self.reserved() {
            Some(reserved) => preimage.extend(reserved.serialize()),
            None => preimage.push(0x00),
        };

        preimage.hash(Some(HashTag::SighashAuthenticable))
    }
}
