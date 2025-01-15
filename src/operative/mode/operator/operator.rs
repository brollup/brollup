use crate::liquidity::provider;
use crate::nns;
use crate::nns::client::NNSClient;
use crate::noist::manager::DKGManager;
use crate::ocli;
use crate::peer::Peer;
use crate::peer::PeerKind;
use crate::tcp;
use crate::tcp::tcp::open_port;
use crate::Network;
use crate::OperatingMode;
use crate::DKG_MANAGER;
use crate::PEER;
use crate::{baked, key::KeyHolder};
use colored::Colorize;
use std::io::{self, BufRead};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
pub async fn run(keys: KeyHolder, _network: Network) {
    let mode = OperatingMode::Operator;

    // 1. Check if this is a liquidity provider.
    if !provider::is_provider(keys.public_key()) {
        eprintln!("{}", "Operator <nsec> does not match.".red());
        return;
    }

    println!("{}", "Initializing operator..");

    // 2. Initialize NNS client.
    let nns_client = NNSClient::new(&keys).await;

    // 3. Initialize NOIST Manager.
    let mut dkg_manager: DKG_MANAGER = match DKGManager::new() {
        Some(manager) => Arc::new(Mutex::new(manager)),
        None => return eprintln!("{}", "Error initializing NOIST manager.".red()),
    };

    // 4. Open port 6272 for incoming connections.
    match open_port().await {
        true => println!("{}", format!("Opened port '{}'.", baked::PORT).green()),
        false => (),
    }

    // 5. Run NNS server.
    {
        let nns_client = nns_client.clone();
        let _ = tokio::spawn(async move {
            let _ = nns::server::run(&nns_client, mode).await;
        });
    }

    // 6. Run TCP server.
    {
        let nns_client = nns_client.clone();
        let dkg_manager = Arc::clone(&dkg_manager);

        let _ = tokio::spawn(async move {
            let _ = tcp::server::run(mode, &nns_client, &keys, &dkg_manager).await;
        });
    }

    // 7. Connect to the coordinator.
    let coordinator: PEER = {
        loop {
            match Peer::connect(
                PeerKind::Coordinator,
                baked::COORDINATOR_WELL_KNOWN,
                &nns_client,
            )
            .await
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

    // 8. CLI
    cli(&mut dkg_manager, &coordinator).await;
}

pub async fn cli(dkg_manager: &mut DKG_MANAGER, _coordinator: &PEER) {
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
            "dkg" => ocli::dkg::command(parts, dkg_manager).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
