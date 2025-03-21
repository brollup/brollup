use super::bitcoin_rpc_error::ValidateRPCError;
use crate::{rpcholder::RPCHolder, Network};
use bitcoin::{Block, BlockHash};
use bitcoincore_rpc::{json::GetBlockchainInfoResult, Auth, Client, RpcApi};

pub fn validate_rpc(rpc_holder: &RPCHolder, network: Network) -> Result<(), ValidateRPCError> {
    let rpc_url = rpc_holder.url();
    let rpc_user = rpc_holder.user();
    let rpc_password = rpc_holder.password();

    let rpc_client = match Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)) {
        Ok(client) => client,
        Err(err) => return Err(ValidateRPCError::RPCErr(err)),
    };

    let blockchain_info: GetBlockchainInfoResult = match rpc_client.get_blockchain_info() {
        Ok(result) => result,
        Err(err) => return Err(ValidateRPCError::RPCErr(err)),
    };

    match blockchain_info.chain {
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

    if blockchain_info.initial_block_download {
        return Err(ValidateRPCError::NotSynced);
    }

    Ok(())
}

pub fn get_chain_height(rpc_holder: &RPCHolder) -> Result<u64, bitcoincore_rpc::Error> {
    let rpc_url = rpc_holder.url();
    let rpc_user = rpc_holder.user();
    let rpc_password = rpc_holder.password();

    let rpc_client = match Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)) {
        Ok(client) => client,
        Err(err) => return Err(err),
    };

    let blockchain_info: GetBlockchainInfoResult = match rpc_client.get_blockchain_info() {
        Ok(result) => result,
        Err(err) => return Err(err),
    };

    let chain_height = blockchain_info.blocks;

    Ok(chain_height)
}

pub fn get_block(
    rpc_holder: &RPCHolder,
    height: u64,
) -> Result<bitcoin::blockdata::block::Block, bitcoincore_rpc::Error> {
    let rpc_url = rpc_holder.url();
    let rpc_user = rpc_holder.user();
    let rpc_password = rpc_holder.password();

    let rpc_client = match Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)) {
        Ok(client) => client,
        Err(err) => return Err(err),
    };

    let block_hash: BlockHash = match rpc_client.get_block_hash(height) {
        Ok(block_hash) => block_hash,
        Err(err) => return Err(err),
    };

    let block: Block = match rpc_client.get_block(&block_hash) {
        Ok(block) => block,
        Err(err) => return Err(err),
    };

    Ok(block)
}
