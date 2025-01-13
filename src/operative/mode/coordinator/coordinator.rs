use crate::nns::client::NNSClient;
use crate::noist::manager::NOISTManager;
use crate::tcp::peer::{Peer, PeerKind};
use crate::tcp::tcp::open_port;
use crate::{baked, key::KeyHolder};
use crate::{ccli, nns, tcp, Network, OperatingMode, NOIST_MANAGER, PEER, PEER_LIST};
use colored::Colorize;
use std::io::{self, BufRead};
use std::sync::Arc;
use std::time::Duration;
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
    let mut noist_manager: NOIST_MANAGER = match NOISTManager::new() {
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
        let noist_manager = Arc::clone(&noist_manager);

        let _ = tokio::spawn(async move {
            let _ = tcp::server::run(mode, &nns_client, &keys, &noist_manager).await;
        });
    }

    // 7. Initialize operator list.
    let operator_list: PEER_LIST = Arc::new(Mutex::new(Vec::<PEER>::new()));

    // 8. Connect to operators.
    for nns_key in baked::OPERATOR_SET.iter() {
        let nns_client = nns_client.clone();
        let operator_list = Arc::clone(&operator_list);

        tokio::spawn(async move {
            let operator: PEER = loop {
                match Peer::connect(PeerKind::Operator, nns_key.to_owned(), &nns_client).await {
                    Ok(connection) => break connection,
                    Err(_) => {
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                }
            };

            let mut _operator_list = operator_list.lock().await;
            _operator_list.push(operator);
        });
    }

    // 9. CLI
    cli(&operator_list, &mut noist_manager).await;
}

pub async fn cli(operator_list: &PEER_LIST, noist_manager: &mut NOIST_MANAGER) {
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
            "noist" => ccli::noist::command(parts, operator_list, noist_manager).await,
            "operator" => ccli::operator::command(operator_list).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
