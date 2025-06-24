/// Enum to represent errors that can occur when decoding a `CommonShortVal` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommonShortValCPEDecodingError {
    SixBitsCollectError,
    DecodeIndexError,
    UncommonIntegerError,
    U8ExtFromBitsError,
}
