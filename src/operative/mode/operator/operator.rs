use crate::epoch_dir::dir::EpochDirectory;
use crate::key::KeyHolder;
use crate::lp_dir::dir::LPDirectory;
use crate::nns;
use crate::nns::client::NNSClient;
use crate::noist::manager::DKGManager;
use crate::ocli;
use crate::peer::Peer;
use crate::peer::PeerKind;
use crate::peer_manager::coordinator_key;
use crate::registery::account::AccountRegistery;
use crate::registery::contract::ContractRegistery;
use crate::rollup_dir::dir::RollupDirectory;
use crate::rpc::bitcoin_rpc::validate_rpc;
use crate::rpcholder::RPCHolder;
use crate::sync::RollupSync;
use crate::tcp;
use crate::tcp::tcp::open_port;
use crate::tcp::tcp::port_number;
use crate::Network;
use crate::OperatingMode;
use crate::ACCOUNT_REGISTERY;
use crate::CONTRACT_REGISTERY;
use crate::DKG_MANAGER;
use crate::EPOCH_DIRECTORY;
use crate::LP_DIRECTORY;
use crate::PEER;
use crate::ROLLUP_DIRECTORY;
use colored::Colorize;
use std::io::{self, BufRead};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
pub async fn run(key_holder: KeyHolder, network: Network, rpc_holder: RPCHolder) {
    let mode = OperatingMode::Operator;

    // #1 Validate Bitcoin RPC.
    if let Err(err) = validate_rpc(&rpc_holder, network) {
        println!("{} {}", "Bitcoin RPC Error: ".red(), err);
        return;
    }

    println!("{}", "Initializing operator..");

    // #2 Initialize Epoch directory.
    let epoch_dir: EPOCH_DIRECTORY = match EpochDirectory::new(network) {
        Some(epoch_dir) => epoch_dir,
        None => {
            println!("{}", "Error initializing epoch directory.".red());
            return;
        }
    };

    // #3 Initialize LP directory.
    let lp_dir: LP_DIRECTORY = match LPDirectory::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing LP directory.".red());
            return;
        }
    };

    // #4 Initialize Account registery.
    let account_registery: ACCOUNT_REGISTERY = match AccountRegistery::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing account registery.".red());
            return;
        }
    };

    // #5 Initialize Contract registery.
    let contract_registery: CONTRACT_REGISTERY = match ContractRegistery::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing contract registery.".red());
            return;
        }
    };

    // #6 Initialize rollup directory.
    let rollup_dir: ROLLUP_DIRECTORY = match RollupDirectory::new(network) {
        Some(dir) => dir,
        None => {
            println!("{}", "Error initializing rollup directory.".red());
            return;
        }
    };

    // #7 Spawn syncer.
    {
        let network = network.clone();
        let key_holder = key_holder.clone();
        let rpc_holder = rpc_holder.clone();
        let epoch_dir = Arc::clone(&epoch_dir);
        let lp_dir = Arc::clone(&lp_dir);
        let account_registery = Arc::clone(&account_registery);
        let contract_registery = Arc::clone(&contract_registery);
        let rollup_dir = Arc::clone(&rollup_dir);

        tokio::spawn(async move {
            let _ = rollup_dir
                .sync(
                    network,
                    &rpc_holder,
                    &key_holder,
                    &epoch_dir,
                    &lp_dir,
                    &account_registery,
                    &contract_registery,
                    None,
                )
                .await;
        });
    }

    println!("{}", "Syncing rollup.");

    // #8 Await rollup to be fully synced.
    rollup_dir.await_sync().await;

    println!("{}", "Syncing complete.");

    // #9 Construct account.
    let account = {
        let _account_registery = account_registery.lock().await;

        match _account_registery.account_by_key_maybe_registered(key_holder.public_key()) {
            Some(account) => account,
            None => {
                println!("{}", "Error constructing account.".red());
                return;
            }
        }
    };

    // #10 Check if this account is a liquidity provider or an operator.
    {
        let is_lp = {
            let _lp_dir = lp_dir.lock().await;
            _lp_dir.is_lp(account)
        };

        let is_operator = {
            let _epoch_dir = epoch_dir.lock().await;
            _epoch_dir.is_operator(network, account)
        };

        if !is_lp || !is_operator {
            eprintln!(
                "{}",
                "This account is not an active liquidity provider or operator.".red()
            );
            return;
        }
    }

    // #11 Initialize NNS client.
    let nns_client = NNSClient::new(&key_holder).await;

    // #12 Open port 6272 for incoming connections.
    match open_port(network).await {
        true => println!(
            "{}",
            format!("Opened port '{}'.", port_number(network)).green()
        ),
        false => (),
    }

    // #13 Run NNS server.
    {
        let nns_client = nns_client.clone();
        let _ = tokio::spawn(async move {
            let _ = nns::server::run(&nns_client, mode).await;
        });
    }

    // #14 Connect to the coordinator.
    let coordinator: PEER = {
        let coordinator_key = coordinator_key(network);

        loop {
            match Peer::connect(network, PeerKind::Coordinator, coordinator_key, &nns_client).await
            {
                Ok(connection) => break connection,
                Err(_) => {
                    println!(
                        "{}",
                        "Failed to connect coordinator. Re-trying in 5..".red()
                    );
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };
        }
    };

    // #15 Initialize DKG Manager.
    let mut dkg_manager: DKG_MANAGER = match DKGManager::new(&lp_dir) {
        Some(manager) => manager,
        None => return eprintln!("{}", "Error initializing DKG manager.".red()),
    };

    // #16 Run TCP server.
    {
        let nns_client = nns_client.clone();
        let dkg_manager = Arc::clone(&dkg_manager);

        let _ = tokio::spawn(async move {
            let _ =
                tcp::server::run(mode, network, &nns_client, &key_holder, &dkg_manager, None).await;
        });
    }

    // #17 CLI.
    cli(&mut dkg_manager, &coordinator).await;
}

pub async fn cli(dkg_manager: &mut DKG_MANAGER, coordinator: &PEER) {
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
            "clear" => ocli::clear::command(),
            "dkg" => ocli::dkg::command(parts, coordinator, dkg_manager).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
