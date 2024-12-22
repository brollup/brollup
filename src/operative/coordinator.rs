use std::sync::Arc;

use colored::Colorize;
use tokio::sync::Mutex;

use crate::{baked, key::KeyHolder, nns_relay::Relay, nns_server, upnp};

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
    match upnp::open_port().await {
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

    println!("{}", "Running coordinator.".green());

    // 1. Background task.
    let _ = tokio::spawn(async move {
        let _ = nns_server::run(&nostr_client).await;
    })
    .await;
}
