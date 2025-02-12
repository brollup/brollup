use super::nonces::NSessionNonces;
use secp::Point;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NSessionRequest {
    msg_sender: Point,
    nonces: NSessionNonces,
}

impl NSessionRequest {
    pub fn new(key: Point, nonces: &NSessionNonces) -> NSessionRequest {
        NSessionRequest {
            msg_sender: key,
            nonces: nonces.to_owned(),
        }
    }

    pub fn msg_sender(&self) -> Point {
        self.msg_sender
    }

    pub fn nonces(&self) -> NSessionNonces {
        self.nonces.clone()
    }
}
