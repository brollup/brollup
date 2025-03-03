use crate::txn::outpoint::Outpoint;
use serde::{Deserialize, Serialize};

/// `CSessionCommitNack` is returned by the coordinator to the msg.senders
/// upon receiving `NSessionCommit` if the commitment fails.
#[derive(Clone, Serialize, Deserialize)]
pub enum CSessionCommitNack {
    SessionLocked,
    AuthErr,
    Overlap,
    Allowance,
    InvalidLiftRemoteKey,
    InvalidLiftOperatorKey,
    MissingLiftOutpoint(),
    InvalidLiftOutpoint(Outpoint),
    InsufficientConnectors,
}
