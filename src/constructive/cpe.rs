use bit_vec::BitVec;

/// Trait for encoding and decoding for compact Bitcoin-DA storage.
pub trait CompactPayloadEncoding: Sized {
    /// Encode the struct into a bitvec.
    fn encode(&self) -> BitVec;
    /// Decode the struct from a bitvec stream returning the struct and the remaining bitvec stream.
    fn decode(data: &BitVec) -> Option<(Self, BitVec)>;
}
