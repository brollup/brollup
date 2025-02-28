use serde::{Deserialize, Serialize};

/// `OSessionOpCovError` is returned by the operators to the coordinator
/// shortly after receiving `CSessionOpCov`s if an issue was encountered.
#[derive(Clone, Serialize, Deserialize)]
pub enum OSessionOpCovError {
    DKGDirHeightErr,
    DKGNonceHeightErr,
    SigningErr,
}
