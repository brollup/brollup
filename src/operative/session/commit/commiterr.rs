use serde::{Deserialize, Serialize};

/// `CSessionCommitError` is returned by the coordinator to the msg.senders
/// upon receiving `NSessionCommit` if the commitment fails.
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
