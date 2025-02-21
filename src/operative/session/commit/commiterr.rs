use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum CSessionCommitError {
    SessionLocked,
    AuthErr,
    Overlap,
    Allowance,
    InvalidLiftRemoteKey,
    InvalidLiftOperatorKey,
    InvalidLiftOutpoint,
    InsufficientConnectors,
}
