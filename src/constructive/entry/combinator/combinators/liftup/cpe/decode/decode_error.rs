use crate::constructive::valtype::val::short_val::cpe::decode::decode_error::ShortValCPEDecodingError;

/// Type for the input iterator position.
type TXInputIterPosition = u32;

/// The error type for decoding a `Liftup` as a CPE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiftupCPEDecodingError {
    NumLiftsCPEDecodeError(ShortValCPEDecodingError),
    NoLiftAtInputIter(TXInputIterPosition),
    LiftReconstructionErrAtInputIter(TXInputIterPosition),
    NoMatchingLiftAtInputIter(TXInputIterPosition),
}
