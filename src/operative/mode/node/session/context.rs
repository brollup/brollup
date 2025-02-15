use crate::{
    entry::{liftup::Liftup, recharge::Recharge},
    valtype::account::Account,
};
use secp::Scalar;

#[derive(Clone)]
pub struct NSessionCtx {
    account: Account,
    secret_key: Scalar,
    liftup: Option<Liftup>,
    recharge: Option<Recharge>,
}
