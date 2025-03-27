use serde::{Deserialize, Serialize};

/// Error type for `Liftup` CPE decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiftupCPEDecodingError {
    // Unable to find a matching `Lift` at the given transaction input iterator position.
    NoLiftAtInputIter(u32),
    // Unable to re-construct `Lift` at the given transaction input iterator position.
    LiftReconstructionErrAtInputIter(u32),
    // Unable to find a matching `Lift` at the given transaction input iterator position.
    NoMatchingLiftAtInputIter(u32),
}
