use crate::{
    entry::{call::Call, liftup::Liftup, recharge::Recharge, reserved::Reserved, vanilla::Vanilla},
    valtype::account::Account,
};
use secp::Scalar;
use serde::{Deserialize, Serialize};

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
