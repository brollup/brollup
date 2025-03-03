use serde::{Deserialize, Serialize};

/// `NSessionUpholdError` is returned by the msg.sender to the coordinator
/// shortly after receiving `CSessionCommitAck`s if there is an issue with creating `NSessionUphold`.
#[derive(Clone, Serialize, Deserialize)]
pub enum NSessionUpholdError {
    CommitAckAuthErr,
    PayloadAuthPartialSignErr,
    VTXOProjectorPartialSignErr,
    ConnectorProjectorPartialSignErr,
    ZKPContigentPartialSignErr,
    UnabletoFindLiftSecretNonces,
    LiftPrevtxoPartialSignErr,
    UnabletoFindConnectorSecretNonces,
    ConnectorPartialSignErr,
    AuthenticableErr,
}
