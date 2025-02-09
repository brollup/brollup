use crate::{
    into::{IntoPoint, IntoScalar},
    musig::session::MusigSessionCtx,
    CSESSION_CTX, DKG_MANAGER,
};
use secp::{Point, Scalar};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use super::stage::CSessionStage;

#[derive(Clone)]
pub struct CSessionCtx {
    dkg_manager: DKG_MANAGER,
    stage: CSessionStage,
    // Remote keys:
    msg_senders: Vec<Point>,
    // Payload Auth:
    payload_auth_nonces: HashMap<Point, (Point, Point)>,
    payload_auth_musig_ctx: Option<MusigSessionCtx>,
    // VTXO projector:
    vtxo_projector_nonces: HashMap<Point, (Point, Point)>,
    vtxo_projector_musig_ctx: Option<MusigSessionCtx>,
    // Connector projector:
    connector_projector_nonces: HashMap<Point, (Point, Point)>,
    connector_projector_musig_ctx: Option<MusigSessionCtx>,
    // ZKP contingent:
    zkp_contingent_nonces: HashMap<Point, (Point, Point)>,
    zkp_contingent_musig_ctx: Option<MusigSessionCtx>,
    // Lift txos:
    lift_prevtxo_musig_ctxes: HashMap<Point, Vec<MusigSessionCtx>>,
    // Connectors:
    connector_txo_musig_ctxes: HashMap<Point, Vec<MusigSessionCtx>>,
}

impl CSessionCtx {
    pub fn construct(dkg_manager: &DKG_MANAGER) -> CSESSION_CTX {
        let session = CSessionCtx {
            dkg_manager: Arc::clone(dkg_manager),
            stage: CSessionStage::Off,
            msg_senders: Vec::<Point>::new(),
            payload_auth_nonces: HashMap::<Point, (Point, Point)>::new(),
            payload_auth_musig_ctx: None,
            vtxo_projector_nonces: HashMap::<Point, (Point, Point)>::new(),
            vtxo_projector_musig_ctx: None,
            connector_projector_nonces: HashMap::<Point, (Point, Point)>::new(),
            connector_projector_musig_ctx: None,
            zkp_contingent_nonces: HashMap::<Point, (Point, Point)>::new(),
            zkp_contingent_musig_ctx: None,
            connector_txo_musig_ctxes: HashMap::<Point, Vec<MusigSessionCtx>>::new(),
            lift_prevtxo_musig_ctxes: HashMap::<Point, Vec<MusigSessionCtx>>::new(),
        };
        Arc::new(Mutex::new(session))
    }

    pub fn payload_auth_nonces(&self) -> HashMap<Point, (Point, Point)> {
        self.payload_auth_nonces.clone()
    }

    pub fn payload_auth_musig_ctx(&self) -> Option<MusigSessionCtx> {
        self.payload_auth_musig_ctx.clone()
    }

    pub fn stage(&self) -> CSessionStage {
        self.stage
    }

    pub fn msg_senders_len(&self) -> usize {
        self.msg_senders.len()
    }

    pub fn msg_senders(&self) -> Vec<Point> {
        self.msg_senders.clone()
    }

    pub fn on(&mut self) {
        self.reset();
        self.stage = CSessionStage::On;
    }

    pub fn add_remote(&mut self, key: [u8; 32], hiding_nonce: Point, binding_nonce: Point) -> bool {
        true
    }

    pub fn lock(&mut self) -> bool {
        self.stage = CSessionStage::Locked;

        let tweak = [0xfeu8; 32].into_scalar().unwrap();

        true
    }

    pub fn ready(&mut self, musig_ctx: &MusigSessionCtx) {
        self.stage = CSessionStage::Ready;

        // self.musig_ctx = Some(musig_ctx.to_owned());
    }

    pub fn insert_partial_sig(&mut self, key: [u8; 32], partial_sig: Scalar) -> bool {
        true
    }

    pub fn full_agg_sig(&self) -> Option<[u8; 64]> {
        None
    }

    pub fn finalized(&mut self) {
        self.stage = CSessionStage::Finalized;
    }

    pub fn off(&mut self) {
        self.stage = CSessionStage::Off;
    }

    pub fn reset(&mut self) {
        self.msg_senders = Vec::<Point>::new();
        self.payload_auth_nonces = HashMap::<Point, (Point, Point)>::new();
        self.payload_auth_musig_ctx = None;
        self.vtxo_projector_nonces = HashMap::<Point, (Point, Point)>::new();
        self.vtxo_projector_musig_ctx = None;
        self.connector_projector_nonces = HashMap::<Point, (Point, Point)>::new();
        self.connector_projector_musig_ctx = None;
        self.zkp_contingent_nonces = HashMap::<Point, (Point, Point)>::new();
        self.zkp_contingent_musig_ctx = None;
        self.connector_txo_musig_ctxes = HashMap::<Point, Vec<MusigSessionCtx>>::new();
        self.lift_prevtxo_musig_ctxes = HashMap::<Point, Vec<MusigSessionCtx>>::new();
    }
}

// HashMap<Point, (Point, Point)>,
