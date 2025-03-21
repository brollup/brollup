use async_trait::async_trait;
use bit_vec::BitVec;

#[derive(Debug, Clone)]
pub enum CPEError {
    RegisteryError,
    IteratorError,
    ConversionError,
}

/// Trait for encoding structs for compact Bitcoin-DA storage.
#[async_trait]
pub trait CompactPayloadEncoding: Sized {
    /// Encode the struct into a bitvec.
    fn encode_cpe(&self) -> BitVec;
}
