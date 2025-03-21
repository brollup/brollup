use crate::registery::registery::REGISTERY;
use async_trait::async_trait;
use bit_vec::BitVec;

#[derive(Debug, Clone)]
pub enum CPEError {
    RegisteryError,
    IteratorError,
    ConversionError,
}

/// Trait for encoding and decoding structs for compact Bitcoin-DA storage.
#[async_trait]
pub trait CompactPayloadEncoding: Sized {
    /// Encode the struct into a bitvec.
    fn encode(&self) -> BitVec;
    /// Decode the struct from a bitvec stream returning the struct and the remaining bitvec stream.
    async fn decode(
        data: &BitVec,
        registery: Option<&REGISTERY>,
    ) -> Result<(Self, BitVec), CPEError>;
}
