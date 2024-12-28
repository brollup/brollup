use crate::tcp_client::{self, Request};
use crate::{baked, key::KeyHolder, nns_relay::Relay, nns_server, tcp_server};
use crate::{noist_vse, tcp, Network, OperatingMode};
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
        let _ = tcp_server::run(&client_list_, mode, &nostr_client_, &keys).await;
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
            "vse" => vse_test(operator_list).await,
            "operators" => handle_operators_command(operator_list).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}

async fn vse_test(operator_list: &PeerList) {
    println!("vse_test");

    let mut connected_operator_key_list = Vec::<[u8; 32]>::new();
    let connected_operator_list: Vec<Peer> = {
        let mut list: Vec<Arc<Mutex<tcp_client::Peer>>> = Vec::<Peer>::new();
        let _operator_list = operator_list.lock().await;

        for (_, peer) in _operator_list.iter().enumerate() {
            let conn = {
                let _peer = peer.lock().await;
                _peer.connection()
            };

            if let Some(_) = conn {
                {
                    let _peer = peer.lock().await;
                    connected_operator_key_list.push(_peer.nns_key());
                }

                list.push(Arc::clone(&peer));
            }
        }
        list
    };

    println!(
        "connected_operator_key_list len: {}",
        connected_operator_key_list.len()
    );

    let mut directory = noist_vse::Directory::new(&connected_operator_key_list);

    for connector_operator in connected_operator_list {
        let map = match connector_operator
            .retrieve_vse_keymap(&connected_operator_key_list)
            .await
        {
            Ok(map) => map,
            Err(_) => continue,
        };

        if !directory.insert(map.clone()) {
            println!("directory insertion failed.");
            return;
        }

        println!("vse retrieved from: {}", hex::encode(map.signer_key()));

        for xxx in map.map().iter() {
            println!("signer {} -> {}", hex::encode(xxx.0), hex::encode(xxx.1));
        }
    }

    if !directory.validate() {
        println!("directory validation failed.");
    } else {
        println!("directory validation passed.");
    }
}

async fn handle_operators_command(operator_list: &PeerList) {
    let _operator_list = operator_list.lock().await;

    for (index, peer) in _operator_list.iter().enumerate() {
        let _peer = peer.lock().await;
        println!(
            "Operator #{} ({}): {}",
            index,
            hex::encode(_peer.nns_key()),
            _peer.addr()
        );
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
