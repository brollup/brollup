use crate::valtype::account::Account;
use serde::{Deserialize, Serialize};

/// `CSessionUpholdONack` (Outer Nack)is returned by the coordinator to the msg.senders
/// shortly after receiving `NSessionUphold`s if there is an issue with one or more `NSessionUphold`s.
#[derive(Clone, Serialize, Deserialize)]
pub enum CSessionUpholdONack {
    BlameMsgSenders(Vec<Account>),
    BlameOperator,
    PayloadAuthSigErr,
    VtxoProjectorSigErr,
    ConnectorProjectorSigErr,
    ZkpContigentSigErr,
    LiftSigErr,
    ConnectorSigErr,
}
