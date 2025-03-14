use crate::{LIFT_WALLET, PEER, VTXO_WALLET};
use secp::{Point, Scalar};

pub async fn command(
    _coordinator: &PEER,
    _lift_wallet: &LIFT_WALLET,
    _vtxo_wallet: &VTXO_WALLET,
    _sk: Scalar,
    _pk: Point,
) {
}
