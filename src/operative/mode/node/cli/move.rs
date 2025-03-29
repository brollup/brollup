use crate::{communicative::peer::peer::PEER, inscriptive::wallet::wallet::WALLET};
use secp::{Point, Scalar};

// move <npub> <amount>
pub async fn move_command(_coordinator: &PEER, _wallet: &WALLET, _sk: Scalar, _pk: Point) {}
