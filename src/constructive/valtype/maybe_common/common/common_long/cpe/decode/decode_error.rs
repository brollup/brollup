/// Enum to represent errors that can occur when decoding a `CommonLongVal` from a bit stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommonLongValCPEDecodingError {
    SevenBitsCollectError,
    DecodeIndexError,
    UncommonIntegerError,
    U8ExtFromBitsError,
}
