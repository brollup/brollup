use crate::{baked, key::KeyHolder, nns_relay::Relay, tcp, tcp_client, OperatingMode};
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;
type SocketList = Arc<Mutex<HashMap<String, (TCPSocket, PeerKind)>>>;

#[derive(Copy, Clone, PartialEq)]
pub enum PeerKind {
    Coordinator,
    Operator,
    Indexer,
    Node,
}

impl PeerKind {
    pub fn as_str(&self) -> &str {
        match self {
            PeerKind::Coordinator => "Coordinator",
            PeerKind::Operator => "Operator",
            PeerKind::Indexer => "Indexer",
            PeerKind::Node => "Node",
        }
    }
}

#[tokio::main]
pub async fn run(keys: KeyHolder, mode: OperatingMode) {
    println!("{}", "Initiating client ..");

    // 1. Inititate Nostr client.
    let nostr_client = {
        let nostr_client = nostr_sdk::Client::new(keys.nostr_key_pair());
        nostr_client.add_default_relay_list().await;
        nostr_client.connect().await;

        Arc::new(Mutex::new(nostr_client))
    };

    // 2. Connect to the coordinator.
    let (coordinator_socket, coordinator_addr) = {
        loop {
            match tcp::connect_nns(baked::COORDINATOR_WELL_KNOWN, &nostr_client).await {
                Ok(connection) => {
                    let socket_addr = {
                        let _connection = connection.lock().await;
                        match _connection.peer_addr() {
                            Ok(socket) => socket,
                            Err(_) => return,
                        }
                    };

                    break (connection, socket_addr);
                }
                Err(_) => {
                    println!("{}", "Failed to connect. Retrying in 5..".red());
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            }
        }
    };

    let peer_list: SocketList = {
        let mut peer_list: HashMap<String, (TCPSocket, PeerKind)> = HashMap::new();
        let coordinator_id = format!("{}:{}", coordinator_addr.ip(), coordinator_addr.port());
        peer_list.insert(coordinator_id, (coordinator_socket, PeerKind::Coordinator));

        Arc::new(Mutex::new(peer_list))
    };

    println!("{}", "Running client.".green());

    cli(&peer_list).await;
}

pub async fn cli(peer_list: &SocketList) {
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
            "conn" => handle_conn_command(peer_list).await,
            "ping" => handle_ping_command(peer_list).await,
            _ => break,
        }
    }
}

fn handle_clear_command() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

async fn handle_conn_command(peer_list: &SocketList) {
    let _peer_list = peer_list.lock().await;

    for (index, (peer_id, (_, peer_kind))) in _peer_list.iter().enumerate() {
        println!("Peer #{}: {} -> {}", index, peer_kind.as_str(), peer_id);
    }
}

async fn handle_ping_command(peer_list: &SocketList) {
    let _peer_list = peer_list.lock().await;

    match _peer_list
        .iter()
        .find(|peer| peer.1 .1 == PeerKind::Coordinator)
    {
        Some(coordinator) => {
            let coordinator_socket = &coordinator.1 .0;
            match tcp_client::ping(coordinator_socket).await {
                Ok(_) => println!("Ponged."),
                Err(_) => println!("Error pinging."),
            }
        }
        None => println!("Coordinator connection dropped."),
    }
}
