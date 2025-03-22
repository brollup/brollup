use bitcoin::hashes::Hash;
use bitcoin::{OutPoint, Txid};

pub trait OutpointExt {
    /// Simple OutPoint constructor.    
    fn simple_new(txid: [u8; 32], vout: u32) -> OutPoint;
    /// Returns the OutPoint as a 36 byte array.
    fn bytes_36(&self) -> [u8; 36];
}

impl OutpointExt for OutPoint {
    fn simple_new(txid: [u8; 32], vout: u32) -> OutPoint {
        OutPoint::new(Txid::from_byte_array(txid), vout)
    }

    fn bytes_36(&self) -> [u8; 36] {
        let mut bytes = [0; 36];
        bytes[..32].copy_from_slice(&self.txid.to_byte_array());
        bytes[32..].copy_from_slice(&self.vout.to_le_bytes());
        bytes
    }
}
