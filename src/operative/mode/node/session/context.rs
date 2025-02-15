use crate::{txo::lift::Lift, valtype::account::Account};
use secp::Scalar;

#[derive(Clone)]
pub struct NSessionCtx {
    account: Account,
    secret_key: Scalar,
    lift_prevtxos: Vec<Lift>,
}
