use crate::epoch::dir::EpochDirectory;
use crate::key::KeyHolder;
use crate::lp::dir::LPDirectory;
use crate::nns;
use crate::nns::client::NNSClient;
use crate::noist::manager::DKGManager;
use crate::ocli;
use crate::peer::Peer;
use crate::peer::PeerKind;
use crate::peer_manager::coordinator_key;
use crate::rpc::bitcoin_rpc::validate_rpc;
use crate::rpcholder::RPCHolder;
use crate::tcp;
use crate::tcp::tcp::open_port;
use crate::tcp::tcp::port_number;
use crate::valtype::account::Account;
use crate::Network;
use crate::OperatingMode;
use crate::DKG_MANAGER;
use crate::EPOCH_DIRECTORY;
use crate::LP_DIRECTORY;
use crate::PEER;
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

    // #2 Initialize Epoch directory.
    let _epoch_dir: EPOCH_DIRECTORY = match EpochDirectory::new(network) {
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

    // #4 Construct account.
    let account = match Account::new(key_holder.public_key(), None) {
        Some(account) => account,
        None => {
            println!("{}", "Error initializing account.".red());
            return;
        }
    };

    // #5 Check if this account is a liquidity provider.
    {
        let _lp_dir = lp_dir.lock().await;
        if let None = _lp_dir.lp(account) {
            eprintln!(
                "{}",
                "This account is not an active liquidity provider.".red()
            );
            return;
        }
    }

    println!("{}", "Initializing operator..");

    // #6 Initialize NNS client.
    let nns_client = NNSClient::new(&key_holder).await;

    // #7 Open port 6272 for incoming connections.
    match open_port(network).await {
        true => println!(
            "{}",
            format!("Opened port '{}'.", port_number(network)).green()
        ),
        false => (),
    }

    // #8 Run NNS server.
    {
        let nns_client = nns_client.clone();
        let _ = tokio::spawn(async move {
            let _ = nns::server::run(&nns_client, mode).await;
        });
    }

    // #9 Connect to the coordinator.
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

    // #10 Initialize DKG Manager.
    let mut dkg_manager: DKG_MANAGER = match DKGManager::new(&lp_dir) {
        Some(manager) => manager,
        None => return eprintln!("{}", "Error initializing DKG manager.".red()),
    };

    // #11 Run TCP server.
    {
        let nns_client = nns_client.clone();
        let dkg_manager = Arc::clone(&dkg_manager);

        let _ = tokio::spawn(async move {
            let _ =
                tcp::server::run(mode, network, &nns_client, &key_holder, &dkg_manager, None).await;
        });
    }

    // #12 CLI
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
