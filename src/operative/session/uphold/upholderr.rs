use serde::{Deserialize, Serialize};

/// `CSessionUpholdError` is returned by the coordinator to the msg.senders
/// shortly after receiving `NSessionUphold`s if one or more `NSessionUphold`s are missing.
#[derive(Clone, Serialize, Deserialize)]
pub enum CSessionUpholdError {
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
    // An uphold missing due to some other participant
    UpholdMissingRedo,
}
