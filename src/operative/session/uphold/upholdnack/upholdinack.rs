use serde::{Deserialize, Serialize};

/// `CSessionUpholdINack` (Inner Nack)is returned by the coordinator to the msg.senders
/// immeduately after receiving `NSessionUphold`s if there is an issue with it.
#[derive(Clone, Serialize, Deserialize)]
pub enum CSessionUpholdINack {
    SessionNotLocked,
    AuthErr,
    InvalidPayloadAuthSig,
    MissingVTXOProjectorSig,
    InvalidVTXOProjectorSig,
    MissingConnectorProjectorSig,
    InvalidConnectorProjectorSig,
    MissingZKPContigentSig,
    InvalidZKPContigentSig,
    MissingLiftSig,
    InvalidLiftSig,
    MissingConnectorSig,
    InvalidConnectorSig,
}
