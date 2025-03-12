use crate::bitcoin_rpc::scan_prevouts;
use crate::nns::client::NNSClient;
use crate::peer::{Peer, PeerKind};
use crate::rpcholder::RPCHolder;
use crate::PEER;
use crate::{baked, key::KeyHolder, OperatingMode};
use crate::{ncli, Network};
use colored::Colorize;
use std::io::{self, BufRead};
use std::time::Duration;

#[tokio::main]
pub async fn run(keys: KeyHolder, _network: Network, _rpc_holder: RPCHolder) {
    let _mode = OperatingMode::Node;

    // RPC
    scan_prevouts();

    println!("{}", "Initializing node..");

    // 1. Initialize NNS client.
    let nns_client = NNSClient::new(&keys).await;

    // 2. Connect to the coordinator.
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
                    println!("{}", "Failed to connect. Re-trying in 5..".red());
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };
        }
    };

    // 3. CLI
    cli(&coordinator, &keys).await;
}

pub async fn cli(coordinator_conn: &PEER, keys: &KeyHolder) {
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
                ncli::covj::command(coordinator_conn, keys.secret_key(), keys.public_key()).await
            }
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
