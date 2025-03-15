use crate::epoch_dir::dir::EpochDirectory;
use crate::lift_sync::sync_lifts;
use crate::lp_dir::dir::LPDirectory;
use crate::nns::client::NNSClient;
use crate::peer::{Peer, PeerKind};
use crate::peer_manager::coordinator_key;
use crate::registery::account::AccountRegistery;
use crate::registery::contract::ContractRegistery;
use crate::rollup_dir::dir::{AwaitSync, RollupDirectory};
use crate::rpc::bitcoin_rpc::validate_rpc;
use crate::rpcholder::RPCHolder;
use crate::valtype::account::Account;
use crate::wallet::lift::LiftWallet;
use crate::wallet::vtxo::VTXOWallet;
use crate::{key::KeyHolder, OperatingMode};
use crate::{
    ncli, Network, ACCOUNT_REGISTERY, CONTRACT_REGISTERY, EPOCH_DIRECTORY, LP_DIRECTORY,
    ROLLUP_DIRECTORY, VTXO_WALLET,
};
use crate::{LIFT_WALLET, PEER};
use colored::Colorize;
use std::io::{self, BufRead};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
pub async fn run(key_holder: KeyHolder, network: Network, rpc_holder: RPCHolder) {
    let _operating_mode = OperatingMode::Node;

    // #1 Validate Bitcoin RPC.
    if let Err(err) = validate_rpc(&rpc_holder, network) {
        println!("{} {}", "Bitcoin RPC Error: ".red(), err);
        return;
    }

    println!("{}", "Initializing node.");

    // #2 Initialize Lift wallet.
    let lift_wallet: LIFT_WALLET = match LiftWallet::new(network) {
        Some(lift_wallet) => lift_wallet,
        None => {
            println!("{}", "Error initializing lift wallet.".red());
            return;
        }
    };

    // #3 Initialize VTXO wallet.
    let vtxo_wallet: VTXO_WALLET = match VTXOWallet::new(network) {
        Some(vtxo_wallet) => vtxo_wallet,
        None => {
            println!("{}", "Error initializing vtxo wallet.".red());
            return;
        }
    };

    // #4 Initialize Epoch directory.
    let epoch_dir: EPOCH_DIRECTORY = match EpochDirectory::new(network) {
        Some(epoch_dir) => epoch_dir,
        None => {
            println!("{}", "Error initializing epoch directory.".red());
            return;
        }
    };

    // #5 Initialize LP directory.
    let _lp_dir: LP_DIRECTORY = match LPDirectory::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing LP directory.".red());
            return;
        }
    };

    // #6 Initialize Account registery.
    let _account_registery: ACCOUNT_REGISTERY = match AccountRegistery::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing account registery.".red());
            return;
        }
    };

    // #7 Initialize Contract registery.
    let _contract_registery: CONTRACT_REGISTERY = match ContractRegistery::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing contract registery.".red());
            return;
        }
    };

    // #8 Initialize rollup directory.
    let rollup_dir: ROLLUP_DIRECTORY = match RollupDirectory::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing rollup directory.".red());
            return;
        }
    };

    // #9 Spawn rollup syncer.
    // TODO

    println!("{}", "Syncing rollup.");

    // # 10 Await rollup to be fully synced.
    rollup_dir.await_sync(&rpc_holder).await;

    println!("{}", "Syncing lifts.");

    // #11 Spawn lift syncer.
    {
        let network = network.clone();
        let key_holder = key_holder.clone();
        let rpc_holder = rpc_holder.clone();
        let epoch_dir = Arc::clone(&epoch_dir);
        let lift_wallet = Arc::clone(&lift_wallet);

        tokio::spawn(async move {
            let _ = sync_lifts(network, &key_holder, &rpc_holder, &epoch_dir, &lift_wallet).await;
        });
    }

    // #12 Construct account.
    let _account = match Account::new(key_holder.public_key(), None) {
        Some(account) => account,
        None => {
            println!("{}", "Error initializing account.".red());
            return;
        }
    };

    // #13 Initialize NNS client.
    let nns_client = NNSClient::new(&key_holder).await;

    // #14 Connect to the coordinator.
    let coordinator: PEER = {
        let coordinator_key = coordinator_key(network);

        loop {
            match Peer::connect(network, PeerKind::Coordinator, coordinator_key, &nns_client).await
            {
                Ok(connection) => break connection,
                Err(_) => {
                    println!("{}", "Failed to connect. Re-trying in 5..".red());
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };
        }
    };

    // #15 CLI.
    cli(&coordinator, &key_holder, &lift_wallet, &vtxo_wallet).await;
}

pub async fn cli(
    coordinator_conn: &PEER,
    key_holder: &KeyHolder,
    lift_wallet: &LIFT_WALLET,
    vtxo_wallet: &VTXO_WALLET,
) {
    println!(
        "{}",
        "Enter command (type help for options, type exit to quit):".cyan()
    );

    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                eprintln!("{}", format!("Invalid line.").yellow());
                continue;
            }
        };

        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            // Main commands:
            "exit" => break,
            "clear" => ncli::clear::command(),
            "conn" => ncli::conn::command(coordinator_conn).await,
            "ping" => ncli::ping::command(coordinator_conn).await,
            "covj" => {
                ncli::covj::command(
                    coordinator_conn,
                    lift_wallet,
                    vtxo_wallet,
                    key_holder.secret_key(),
                    key_holder.public_key(),
                )
                .await
            }
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
