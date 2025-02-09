use super::nonces::SessionNonces;
use secp::Point;

#[derive(Clone)]
pub struct SessionRequest {
    msg_sender: Point,
    nonces: SessionNonces,
}

impl SessionRequest {
    pub fn new(key: Point, nonces: &SessionNonces) -> SessionRequest {
        SessionRequest {
            msg_sender: key,
            nonces: nonces.to_owned(),
        }
    }
}
