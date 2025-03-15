use crate::epoch::dir::EpochDirectory;
use crate::lp::dir::LPDirectory;
use crate::nns::client::NNSClient;
use crate::peer::{Peer, PeerKind};
use crate::peer_manager::coordinator_key;
use crate::rpc::bitcoin_rpc::validate_rpc;
use crate::rpcholder::RPCHolder;
use crate::scanner::scan_lifts;
use crate::valtype::account::Account;
use crate::wallet::lift::LiftWallet;
use crate::wallet::vtxo::VTXOWallet;
use crate::{key::KeyHolder, OperatingMode};
use crate::{ncli, Network, EPOCH_DIRECTORY, LP_DIRECTORY, VTXO_WALLET};
use crate::{LIFT_WALLET, PEER};
use colored::Colorize;
use std::io::{self, BufRead};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
pub async fn run(key_holder: KeyHolder, network: Network, rpc_holder: RPCHolder) {
    let _operating_mode = OperatingMode::Node;

    println!("{}", "Initializing node..");

    // #1 Validate Bitcoin RPC.
    if let Err(err) = validate_rpc(&rpc_holder, network) {
        println!("{} {}", "Bitcoin RPC Error: ".red(), err);
        return;
    }

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

    // #6 Construct account.
    let _account = match Account::new(key_holder.public_key(), None) {
        Some(account) => account,
        None => {
            println!("{}", "Error initializing account.".red());
            return;
        }
    };

    // #7 Initialize NNS client.
    let nns_client = NNSClient::new(&key_holder).await;

    // #8 Connect to the coordinator.
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

    // #9 Scan lifts.
    {
        let network = network.clone();
        let key_holder = key_holder.clone();
        let rpc_holder = rpc_holder.clone();
        let epoch_dir = Arc::clone(&epoch_dir);
        let lift_wallet = Arc::clone(&lift_wallet);

        tokio::spawn(async move {
            let _ = scan_lifts(network, &key_holder, &rpc_holder, &epoch_dir, &lift_wallet).await;
        });
    }

    // #10 CLI.
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
