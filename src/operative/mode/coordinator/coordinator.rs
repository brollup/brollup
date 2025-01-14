use crate::nns::client::NNSClient;
use crate::noist::manager::DKGManager;
use crate::peer::PeerKind;
use crate::peer_manager::PeerManager;
use crate::tcp::tcp::open_port;
use crate::{baked, key::KeyHolder};
use crate::{ccli, nns, tcp, Network, OperatingMode, DKG_MANAGER, PEER_MANAGER};
use colored::Colorize;
use std::io::{self, BufRead};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
pub async fn run(keys: KeyHolder, _network: Network) {
    let mode = OperatingMode::Coordinator;

    if keys.public_key() != baked::COORDINATOR_WELL_KNOWN {
        eprintln!("{}", "Coordinator <nsec> does not match.".red());
        return;
    }

    println!("{}", "Initializing coordinator..");

    // 1. Initialize NNS client.
    let nns_client = NNSClient::new(&keys).await;

    // 2.

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

    // 7. Initialize peer manager.
    let operator_set = baked::OPERATOR_SET.to_vec();
    let mut peer_manager: PEER_MANAGER =
        match PeerManager::new(&nns_client, PeerKind::Operator, &operator_set, 1).await {
            Some(manager) => Arc::new(Mutex::new(manager)),
            None => return eprintln!("{}", "Error initializing Peer manager.".red()),
        };

    // 9. CLI
    cli(&mut peer_manager, &mut dkg_manager).await;
}

pub async fn cli(peer_manager: &mut PEER_MANAGER, dkg_manager: &mut DKG_MANAGER) {
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
            "clear" => ccli::clear::command(),
            "dkg" => ccli::dkg::command(parts, peer_manager, dkg_manager).await,
            "operator" => ccli::operator::command(peer_manager).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
