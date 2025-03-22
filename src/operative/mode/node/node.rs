use crate::entity::account::Account;
use crate::epoch_dir::dir::EpochDirectory;
use crate::lp_dir::dir::LPDirectory;
use crate::nns::client::NNSClient;
use crate::peer::{Peer, PeerKind};
use crate::peer_manager::coordinator_key;
use crate::registery::account_registery::ACCOUNT_REGISTERY;
use crate::registery::registery::{Registery, REGISTERY};
use crate::rollup_dir::dir::RollupDirectory;
use crate::rpc::bitcoin_rpc::validate_rpc;
use crate::rpcholder::RPCHolder;
use crate::sync::RollupSync;
use crate::utxoset::utxoset::{UTXOSet, UTXO_SET};
use crate::wallet::wallet::{Wallet, WALLET};
use crate::{key::KeyHolder, OperatingMode};
use crate::{ncli, Network, EPOCH_DIRECTORY, LP_DIRECTORY, PEER, ROLLUP_DIRECTORY};
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

    // #2 Initialize  wallet.
    let wallet: WALLET = match Wallet::new(network, key_holder.public_key()) {
        Some(wallet) => wallet,
        None => {
            println!("{}", "Error initializing wallet.".red());
            return;
        }
    };

    // #3 Initialize Epoch directory.
    let epoch_dir: EPOCH_DIRECTORY = match EpochDirectory::new(network) {
        Some(epoch_dir) => epoch_dir,
        None => {
            println!("{}", "Error initializing epoch directory.".red());
            return;
        }
    };

    // #4 Initialize LP directory.
    let lp_dir: LP_DIRECTORY = match LPDirectory::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing LP directory.".red());
            return;
        }
    };

    // #5 Initialize Registery.
    let registery: REGISTERY = match Registery::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing registery.".red());
            return;
        }
    };

    // #6 Initialize UTXO set.
    let utxoset: UTXO_SET = match UTXOSet::new(network) {
        Some(utxoset) => utxoset,
        None => {
            println!("{}", "Error initializing utxoset.".red());
            return;
        }
    };

    // #7 Initialize rollup directory.
    let rollup_dir: ROLLUP_DIRECTORY = match RollupDirectory::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing rollup directory.".red());
            return;
        }
    };

    // #8 Spawn syncer
    {
        let network = network.clone();
        let key_holder = key_holder.clone();
        let rpc_holder = rpc_holder.clone();
        let epoch_dir = Arc::clone(&epoch_dir);
        let lp_dir = Arc::clone(&lp_dir);
        let registery = Arc::clone(&registery);
        let wallet = Arc::clone(&wallet);
        let rollup_dir = Arc::clone(&rollup_dir);
        let utxoset = Arc::clone(&utxoset);
        tokio::spawn(async move {
            let _ = rollup_dir
                .sync(
                    network,
                    &rpc_holder,
                    &key_holder,
                    &epoch_dir,
                    &lp_dir,
                    &registery,
                    Some(&wallet),
                    &utxoset,
                )
                .await;
        });
    }

    println!("{}", "Syncing rollup.");

    // #9 Wait until rollup to be synced to the latest Bitcoin chain tip.
    rollup_dir.await_sync().await;

    println!("{}", "Syncing complete.");

    // #10 Construct account.
    let account = {
        let account_registery: ACCOUNT_REGISTERY = {
            let _registery = registery.lock().await;
            _registery.account_registery()
        };

        let _account_registery = account_registery.lock().await;

        match _account_registery.account_by_key_maybe_registered(key_holder.public_key()) {
            Some(account) => account,
            None => {
                println!("{}", "Error constructing account.".red());
                return;
            }
        }
    };

    // #11 Initialize NNS client.
    let nns_client = NNSClient::new(&key_holder).await;

    // #12 Connect to the coordinator.
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

    // #13 CLI.
    cli(
        network,
        &coordinator,
        &key_holder,
        &account,
        &wallet,
        &epoch_dir,
    )
    .await;
}

pub async fn cli(
    network: Network,
    coordinator_conn: &PEER,
    key_holder: &KeyHolder,
    _account: &Account,
    wallet: &WALLET,
    epoch_dir: &EPOCH_DIRECTORY,
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
            "clear" => ncli::clear::clear_command(),
            "conn" => ncli::conn::conn_command(coordinator_conn).await,
            "ping" => ncli::ping::ping_command(coordinator_conn).await,
            "npub" => ncli::npub::npub_command(key_holder).await,
            "addr" => ncli::addr::addr_command(network, epoch_dir, key_holder).await,
            "lift" => ncli::lift::lift_command(wallet, epoch_dir, network, key_holder, parts).await,
            "move" => {
                ncli::r#move::move_command(
                    coordinator_conn,
                    wallet,
                    key_holder.secret_key(),
                    key_holder.public_key(),
                )
                .await
            }
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
