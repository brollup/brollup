use crate::tcp_client;
use crate::{baked, key::KeyHolder, nns_relay::Relay, nns_server, tcp_server};
use crate::{tcp, Network, OperatingMode};
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

type Peer = Arc<Mutex<tcp_client::Peer>>;
type PeerList = Arc<Mutex<Vec<Peer>>>;

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;
type SocketList = Arc<Mutex<HashMap<String, TCPSocket>>>;

#[tokio::main]
pub async fn run(keys: KeyHolder, _network: Network) {
    let mode = OperatingMode::Coordinator;

    if keys.public_key() != baked::COORDINATOR_WELL_KNOWN {
        eprintln!("{}", "Coordinator <nsec> does not match.".red());
        return;
    }

    println!("{}", "Initializing coordinator..");

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
        let _ = tcp_server::run(&client_list_, mode, &nostr_client_).await;
    });

    // 5. Connect to operators.
    let operator_list: PeerList = Arc::new(Mutex::new(Vec::<Peer>::new()));
    for nns_key in baked::OPERATOR_SET.iter() {
        let nostr_client = Arc::clone(&nostr_client);
        let operator_list = Arc::clone(&operator_list);

        tokio::spawn(async move {
            let operator: Peer = loop {
                match tcp_client::Peer::connect(
                    tcp_client::PeerKind::Operator,
                    nns_key.to_owned(),
                    &nostr_client,
                )
                .await
                {
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

    println!("{}", "Running coordinator.".green());

    // CLI
    println!(
        "{}",
        "Enter command (type help for options, type exit to quit):".cyan()
    );

    cli(&client_list, &operator_list).await;
}

pub async fn cli(client_list: &SocketList, operator_list: &PeerList) {
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
            "operators" => handle_operators_command(operator_list).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}

async fn handle_operators_command(operator_list: &PeerList) {
    let _operator_list = operator_list.lock().await;

    for (index, peer) in _operator_list.iter().enumerate() {
        let _peer = peer.lock().await;
        println!("Operator #{}: {}", index, _peer.addr());
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
