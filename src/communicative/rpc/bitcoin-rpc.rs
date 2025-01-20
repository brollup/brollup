use bitcoincore_rpc::{Auth, Client, RpcApi};

pub fn bitcoin_rpc() {
    let rpc_url = "http://209.97.150.144:38332";
    let rpc_user = "brollup";
    let rpc_password = "brollup";

    let rpc = Client::new(
        rpc_url,
        Auth::UserPass(rpc_user.to_string(), rpc_password.to_string()),
    )
    .unwrap();

    let blockchain_info = rpc.get_blockchain_info().unwrap();
    println!("Blockchain Info: {:?}", blockchain_info);

    let best_block_hash = rpc.get_best_block_hash().unwrap();

    println!("Best Block Hash: {:?}", best_block_hash);
}
