use std::collections::HashMap;

use super::{nonces::NSessionNonces, request::NSessionRequest};
use crate::{
    entry::{call::Call, liftup::Liftup, recharge::Recharge, reserved::Reserved, vanilla::Vanilla},
    into::IntoScalar,
    schnorr,
    txo::lift::Lift,
    valtype::account::Account,
};
use secp::{Point, Scalar};
use serde::{Deserialize, Serialize};

pub const CONNECTORS_EXTRA_IN: u8 = 10;

#[derive(Clone, Serialize, Deserialize)]
pub enum MainEntry {
    Vanilla(Vanilla),
    Call(Call),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NSessionCtx {
    account: Account,
    secret_key: Scalar,
    liftup: Option<Liftup>,
    recharge: Option<Recharge>,
    main: MainEntry,
    reserved: Option<Reserved>,
}

impl NSessionCtx {
    pub fn new_vanilla(
        account: Account,
        secret_key: Scalar,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
        vanilla: Vanilla,
    ) -> Option<NSessionCtx> {
        let public_key = secret_key.base_point_mul();

        if account.key() != public_key || account.is_odd_key() {
            return None;
        }

        let ctx = NSessionCtx {
            account,
            secret_key,
            liftup,
            recharge,
            main: MainEntry::Vanilla(vanilla),
            reserved: None,
        };

        Some(ctx)
    }

    pub fn new_call(
        account: Account,
        secret_key: Scalar,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
        call: Call,
    ) -> Option<NSessionCtx> {
        let public_key = secret_key.base_point_mul();

        if account.key() != public_key || account.is_odd_key() {
            return None;
        }

        let ctx = NSessionCtx {
            account,
            secret_key,
            liftup,
            recharge,
            main: MainEntry::Call(call),
            reserved: None,
        };

        Some(ctx)
    }

    pub fn account(&self) -> Account {
        self.account
    }

    pub fn liftup(&self) -> Option<Liftup> {
        self.liftup.clone()
    }

    pub fn recharge(&self) -> Option<Recharge> {
        self.recharge.clone()
    }

    pub fn main_entry(&self) -> MainEntry {
        self.main.clone()
    }

    fn gen_nonce(&self) -> Option<((Scalar, Scalar), (Point, Point))> {
        let hiding_secret_nonce = schnorr::generate_secret().into_scalar().ok()?;
        let binding_secret_nonce = schnorr::generate_secret().into_scalar().ok()?;

        let hiding_public_nonce = hiding_secret_nonce.base_point_mul();
        let binding_public_nonce = binding_secret_nonce.base_point_mul();

        Some((
            (hiding_secret_nonce, binding_secret_nonce),
            (hiding_public_nonce, binding_public_nonce),
        ))
    }

    // TODO:
    fn num_connectors(&self) -> u8 {
        3 as u8 + CONNECTORS_EXTRA_IN
    }

    pub fn into_request(&self) -> Option<NSessionRequest> {
        let secret_key = self.secret_key;

        // Collect common nonces:
        let (payload_auth_secret_nonces, payload_auth_public_nonces) = self.gen_nonce()?;
        let (vtxo_projector_secret_nonces, vtxo_projector_public_nonces) = self.gen_nonce()?;
        let (connector_projector_secret_nonces, connector_projector_public_nonces) =
            self.gen_nonce()?;
        let (zkp_contingent_secret_nonces, zkp_contingent_public_nonces) = self.gen_nonce()?;

        // Collect lift nonces
        let mut lift_prevtxo_secret_nonces = HashMap::<Lift, (Scalar, Scalar)>::new();
        let mut lift_prevtxo_public_nonces = HashMap::<Lift, (Point, Point)>::new();

        if let Some(liftup) = &self.liftup {
            for lift in liftup.lifts().iter() {
                let (secret_nonces, public_nonces) = self.gen_nonce()?;

                lift_prevtxo_secret_nonces.insert(lift.to_owned(), secret_nonces);
                lift_prevtxo_public_nonces.insert(lift.to_owned(), public_nonces);
            }
        }

        // Collect connector nonces
        let mut connector_txo_secret_nonces = Vec::<(Scalar, Scalar)>::new();
        let mut connector_txo_public_nonces = Vec::<(Point, Point)>::new();

        let num_connectors = self.num_connectors();

        for _ in 0..num_connectors {
            let (secret_nonces, public_nonces) = self.gen_nonce()?;
            connector_txo_secret_nonces.push(secret_nonces);
            connector_txo_public_nonces.push(public_nonces);
        }

        let session_nonces = NSessionNonces::new(
            payload_auth_public_nonces.0,
            payload_auth_public_nonces.1,
            vtxo_projector_public_nonces.0,
            vtxo_projector_public_nonces.1,
            connector_projector_public_nonces.0,
            connector_projector_public_nonces.1,
            zkp_contingent_public_nonces.0,
            zkp_contingent_public_nonces.1,
            &lift_prevtxo_public_nonces,
            &connector_txo_public_nonces,
        );

        let session_request = NSessionRequest::new(self.account(), &session_nonces);

        Some(session_request)
    }
}
