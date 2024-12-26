use crate::tcp_client;
use crate::tcp_client::Request;
use crate::Network;
use crate::{baked, key::KeyHolder, nns_relay::Relay, OperatingMode};
use colored::Colorize;
use std::io::{self, BufRead, Write};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

type Peer = Arc<Mutex<tcp_client::Peer>>;

#[tokio::main]
pub async fn run(keys: KeyHolder, _network: Network) {
    let _mode = OperatingMode::Coordinator;

    println!("{}", "Initializing node..");

    // 1. Inititate Nostr client.
    let nostr_client = {
        let nostr_client = nostr_sdk::Client::new(keys.nostr_key_pair());
        nostr_client.add_default_relay_list().await;
        nostr_client.connect().await;

        Arc::new(Mutex::new(nostr_client))
    };

    // 2. Connect to the coordinator.
    let coordinator: Peer = {
        loop {
            match tcp_client::Peer::connect(
                tcp_client::PeerKind::Coordinator,
                baked::COORDINATOR_WELL_KNOWN,
                &nostr_client,
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

    println!(
        "{}",
        "Enter command (type help for options, type exit to quit):".cyan()
    );

    cli(&coordinator).await;
}

pub async fn cli(coordinator_conn: &Peer) {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            // Main commands:
            "exit" => break,
            "clear" => handle_clear_command(),
            "conn" => handle_conn_command(coordinator_conn).await,
            "ping" => handle_ping_command(coordinator_conn).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}

fn handle_clear_command() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

async fn handle_conn_command(coordinator: &Peer) {
    let _coordinator = coordinator.lock().await;

    match _coordinator.connection() {
        Some(_) => {
            let addr: String = _coordinator.addr();
            println!("Alive: {}", addr);
        }
        None => {
            println!("Dead.")
        }
    }
}

async fn handle_ping_command(coordinator: &Peer) {
    match coordinator.ping().await {
        Ok(duration) => println!("{} ms", duration.as_millis()),
        Err(_) => println!("Error pinging."),
    }
}
