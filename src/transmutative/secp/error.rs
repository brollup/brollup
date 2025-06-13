/// Errors for the secp module.
#[derive(Debug)]
pub enum SecpError {
    InvalidSignature,
    InvalidScalar,
    InvalidPoint,
    SignatureParseError,
}
