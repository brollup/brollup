use async_trait::async_trait;
use bit_vec::BitVec;

/// Trait for encoding structs for compact Bitcoin-DA storage.
#[async_trait]
pub trait CompactPayloadEncoding {
    /// Encode the struct into a bitvec.
    fn encode_cpe(&self) -> BitVec;
}

// Compact payload decoding is implemented individually for each struct that implements `CompactPayloadEncoding`, rather than using a trait.
