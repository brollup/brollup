use serde::{Deserialize, Serialize};

/// Error type for `CommonNum` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaybeCommonCPEDecodingError {
    // Bit stream iteration error.
    BitStreamIteratorError,
    // Short uncommon value CPE decoding error.
    UncommonShortValCPEDecodingError(ShortValCPEDecodingError),
    // Long uncommon value CPE decoding error.
    UncommonLongValCPEDecodingError(LongValCPEDecodingError),
    // Common short value CPE decoding error.
    CommonShortValCPEDecodingError(CommonShortValCPEDecodingError),
    // Common long value CPE decoding error.
    CommonLongValCPEDecodingError(CommonLongValCPEDecodingError),
}

/// Error type for `AtomicVal` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AtomicValCPEDecodingError {
    // Bit stream iteration error.
    BitStreamIteratorError,
    // U8 bitcodec error.
    U8BitCodecError,
}

/// Error type for `ShortVal` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShortValCPEDecodingError {
    // Bit stream iteration error.
    BitStreamIteratorError,
    // Short value conversion error.
    ShortValConversionError,
    // Common integer decoding error.
    CommonIntDecodingError,
}

/// Error type for `LongVal` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LongValCPEDecodingError {
    // Bit stream iteration error.
    BitStreamIteratorError,
    // Long value conversion error.
    LongValConversionError,
}

/// Error type for `CommonShortVal` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommonShortValCPEDecodingError {
    // Bit stream iteration error.
    BitStreamIteratorError,
    // U8 bitcodec error.
    U8BitCodecError,
    // Uncommon integer error.
    UncommonInteger,
}

/// Error type for `CommonLongVal` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommonLongValCPEDecodingError {
    // Bit stream iteration error.
    BitStreamIteratorError,
    // U8 bitcodec error.
    U8BitCodecError,
    // Uncommon integer error.
    UncommonInteger,
}
