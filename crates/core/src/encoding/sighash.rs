pub trait Sighash {
    fn sighash(&self, prev_state_hash: [u8; 32]) -> [u8; 32];
}