use crate::tcp;
use crate::tcp_server;
use crate::Network;
use crate::OperatingMode;
use crate::{baked, key::KeyHolder, nns_relay::Relay, nns_server};
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use tokio::sync::Mutex;

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;
type SocketList = Arc<Mutex<HashMap<String, TCPSocket>>>;

#[tokio::main]
pub async fn run(keys: KeyHolder, _network: Network) {
    let mode = OperatingMode::Coordinator;

    if !baked::OPERATOR_SET.contains(&keys.public_key()) {
        eprintln!("{}", "Operator <nsec> does not match.".red());
        return;
    }

    println!("{}", "Initializing operator..");

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
    let nostr_client_ = Arc::clone(&nostr_client);
    let _ = tokio::spawn(async move {
        let _ = nns_server::run(&nostr_client_, mode).await;
    });

    let client_list: SocketList = {
        let client_list: HashMap<String, TCPSocket> = HashMap::new();

        Arc::new(Mutex::new(client_list))
    };

    // 4. Run TCP server.
    let client_list_ = Arc::clone(&client_list);
    let nostr_client_ = Arc::clone(&nostr_client);
    let _ = tokio::spawn(async move {
        let _ = tcp_server::run(&client_list_, mode, &nostr_client_, &keys).await;
    });

    // CLI
    println!(
        "{}",
        "Enter command (type help for options, type exit to quit):".cyan()
    );

    cli(&client_list).await;
}

pub async fn cli(client_list: &SocketList) {
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
            "clients" => handle_clients_command(client_list).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}

async fn handle_clients_command(client_list: &SocketList) {
    let _client_list = client_list.lock().await;

    for (index, (client_id, _)) in _client_list.iter().enumerate() {
        println!("Client #{}: {}", index, client_id);
    }
}

fn handle_clear_command() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}
