use colored::Colorize;
use std::io::{self, BufRead};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::tcp;
use crate::{baked, key::KeyHolder, nns_relay::Relay, nns_server, tcp_server};

#[tokio::main]
pub async fn run(keys: KeyHolder) {
    if keys.public_key() != baked::COORDINATOR_WELL_KNOWN {
        eprintln!("{}", "Coordinator <nsec> does not match.".red());
        return;
    }

    println!("{}", "Initiating coordinator ..");

    // 1. Inititate Nostr client.
    let nostr_client = {
        let nostr_client = nostr_sdk::Client::new(keys.nostr_key_pair());
        nostr_client.add_default_relay_list().await;
        nostr_client.connect().await;

        Arc::new(Mutex::new(nostr_client))
    };

    // 2. Open port `6272` for incoming connections.
    match tcp::open_port().await {
        true => {
            println!("{}", format!("Opened port '{}'.", baked::PORT).green());
        }
        false => {
            println!(
                "{}",
                format!(
                    "Failed to open port '{}'. Ignore this warning if the port is already open.",
                    baked::PORT
                )
                .yellow()
            );
            //return;
        }
    }

    // 3. Run NNS server.
    let _ = tokio::spawn(async move {
        let _ = nns_server::run(&nostr_client).await;
    });

    // 4. Run TCP server.
    let _ = tokio::spawn(async move {
        let _ = tcp_server::run().await;
    });

    println!("{}", "Running coordinator.".green());

    // CLI
    cli().await;
}

pub async fn cli() {
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
            _ => break,
        }
    }
}
