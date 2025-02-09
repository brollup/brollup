use crate::txn::outpoint::Outpoint;
use secp::Point;
use std::collections::HashMap;

#[derive(Clone)]
pub struct SessionNonces {
    // Payload auth nonces (hiding & binding):
    payload_auth_nonces: (Point, Point),
    // VTXO projector nonces (hiding & binding):
    vtxo_projector_nonces: (Point, Point),
    // Connector projector nonces (hiding & binding):
    connector_projector_nonces: (Point, Point),
    // ZKP contingent nonces (hiding & binding):
    zkp_contingent_nonces: (Point, Point),
    // Lift prevtxo nonces (outpoint -> hiding & binding):
    lift_txo_nonces: HashMap<Outpoint, (Point, Point)>,
    // Connector txo nonces (hiding & binding):
    connector_txo_nonces: Vec<(Point, Point)>,
}

impl SessionNonces {
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
        lift_txo_nonces: &HashMap<Outpoint, (Point, Point)>,
        connector_txo_nonces: &Vec<(Point, Point)>,
    ) -> SessionNonces {
        SessionNonces {
            payload_auth_nonces: (payload_auth_hiding_nonce, payload_auth_binding_nonce),
            vtxo_projector_nonces: (vtxo_projector_hiding_nonce, vtxo_projector_binding_nonce),
            connector_projector_nonces: (
                connector_projector_hiding_nonce,
                connector_projector_binding_nonce,
            ),
            zkp_contingent_nonces: (zkp_contingent_hiding_nonce, zkp_contingent_binding_nonce),
            lift_txo_nonces: lift_txo_nonces.to_owned(),
            connector_txo_nonces: connector_txo_nonces.to_owned(),
        }
    }
}
