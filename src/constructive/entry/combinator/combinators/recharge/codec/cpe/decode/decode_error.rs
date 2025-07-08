/// The error type for decoding a `Recharge` from a compact bit vector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RechargeCPEDecodingError {
    RechargeConstructionError,
    NoRechargeableVTXOsFound,
}
