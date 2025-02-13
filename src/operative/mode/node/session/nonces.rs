use crate::txo::lift::Lift;
use secp::Point;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct NSessionNonces {
    // Payload auth nonces (hiding & binding):
    payload_auth_nonces: (Point, Point),
    // VTXO projector nonces (hiding & binding):
    vtxo_projector_nonces: (Point, Point),
    // Connector projector nonces (hiding & binding):
    connector_projector_nonces: (Point, Point),
    // ZKP contingent nonces (hiding & binding):
    zkp_contingent_nonces: (Point, Point),
    // Lift prevtxo nonces (Lift -> hiding & binding):
    lift_prevtxo_nonces: HashMap<Lift, (Point, Point)>,
    // Connector txo nonces (hiding & binding):
    connector_txo_nonces: Vec<(Point, Point)>,
}

impl NSessionNonces {
    pub fn new(
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
    ) -> NSessionNonces {
        NSessionNonces {
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
