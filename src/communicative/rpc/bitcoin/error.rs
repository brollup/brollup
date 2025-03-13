use std::fmt;

#[derive(Debug)]
pub enum ValidateRPCError {
    WrongChain,
    NotSynced,
    RPCErr(bitcoincore_rpc::Error),
}

impl fmt::Display for ValidateRPCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidateRPCError::WrongChain => write!(f, "Wrong chain."),
            ValidateRPCError::NotSynced => write!(f, "Node is not fully synced yet."),
            ValidateRPCError::RPCErr(err) => write!(f, "RPC error: {}", err),
        }
    }
}
