use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum NSessionUpholdError {
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
