use async_trait::async_trait;
use bit_vec::BitVec;

/// Error type for compact payload decoding.
#[derive(Debug, Clone)]
pub enum CPEDecodingError {
    RegisteryError,
    BitVecIteratorError,
    ConversionError,
    TxInIteratorErrorAt(u32),
    TxOutIteratorErrorAt(u32),
    InvalidLiftTxInAt(u32),
}

/// Trait for encoding structs for compact Bitcoin-DA storage.
#[async_trait]
pub trait CompactPayloadEncoding {
    /// Encode the struct into a bitvec.
    fn encode_cpe(&self) -> BitVec;
}
