use serde::{Deserialize, Serialize};

/// Error type for `CommonNum` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaybeCommonCPEDecodingError {
    // Bit stream iteration error.
    BitStreamIteratorError,
    // Uncommon integer error.
    UncommonInteger,
    // Common value CPE decoding error.
    CommonValCPEDecodingError,
    // Short uncommon value CPE decoding error.
    ShortUncommonValCPEDecodingError,
    // Long uncommon value CPE decoding error.
    LongUncommonValCPEDecodingError,
}

/// Error type for `AtomicVal` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AtomicValCPEDecodingError {
    // Bit stream iteration error.
    BitStreamIteratorError,
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