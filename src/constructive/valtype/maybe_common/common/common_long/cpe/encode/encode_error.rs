/// Enum to represent errors that can occur when encoding a `CommonLongVal` into a bit vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommonLongValCPEEncodeError {
    U8ExtToBitsError,
}
