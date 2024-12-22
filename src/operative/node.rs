use std::{sync::Arc, time::Duration};

use colored::Colorize;
use tokio::sync::Mutex;

use crate::{baked, key::KeyHolder, nns_relay::Relay, tcp};

#[tokio::main]
pub async fn run(keys: KeyHolder) {
    println!("{}", "Initiating client ..");

    // 1. Inititate Nostr client.
    let nostr_client = {
        let nostr_client = nostr_sdk::Client::new(keys.nostr_key_pair());
        nostr_client.add_default_relay_list().await;
        nostr_client.connect().await;

        Arc::new(Mutex::new(nostr_client))
    };

    // 2. Connect coordinator
    let _coordinator_connection = {
        loop {
            match tcp::connect_nns(baked::COORDINATOR_WELL_KNOWN, &nostr_client).await {
                Some(connection) => break connection,
                None => {
                    println!("{}", "Failed to connect. Retrying in 5..".red());
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            }
        }
    };

    println!("{}", "Running client.".green());
}
