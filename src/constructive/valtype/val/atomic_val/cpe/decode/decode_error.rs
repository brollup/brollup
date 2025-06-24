/// Enum to represent errors that can occur when decoding an `AtomicVal` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AtomicValCPEDecodingError {
    CollectBitsizeNumberBitsError,
    U8ExtFromBitsError,
}
