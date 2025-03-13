use crate::{LIFT_WALLET, PEER};
use secp::{Point, Scalar};

pub async fn command(_coordinator: &PEER, _lift_wallet: &LIFT_WALLET, _sk: Scalar, _pk: Point) {}
