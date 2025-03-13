use super::bitcoin_rpc_error::ValidateRPCError;
use crate::{rpcholder::RPCHolder, Network};
use bitcoincore_rpc::{bitcoin, json::GetBlockchainInfoResult, Auth, Client, RpcApi};

pub async fn validate_rpc(
    rpc_holder: &RPCHolder,
    network: Network,
) -> Result<(), ValidateRPCError> {
    let rpc_url = rpc_holder.url();
    let rpc_user = rpc_holder.user();
    let rpc_password = rpc_holder.password();

    let rpc_client = match Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)) {
        Ok(client) => client,
        Err(err) => return Err(ValidateRPCError::RPCErr(err)),
    };

    let rpc_result: GetBlockchainInfoResult = match rpc_client.get_blockchain_info() {
        Ok(result) => result,
        Err(err) => return Err(ValidateRPCError::RPCErr(err)),
    };

    match rpc_result.chain {
        bitcoin::network::Network::Bitcoin => {
            if network != Network::Mainnet {
                return Err(ValidateRPCError::WrongChain);
            }
        }
        bitcoin::network::Network::Signet => {
            if network != Network::Signet {
                return Err(ValidateRPCError::WrongChain);
            }
        }
        _ => return Err(ValidateRPCError::WrongChain),
    };

    if rpc_result.initial_block_download {
        return Err(ValidateRPCError::NotSynced);
    }

    Ok(())
}
