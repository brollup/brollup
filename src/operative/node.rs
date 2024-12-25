use crate::tcp_client::{uptime, Peer};
use crate::PeerKind;
use crate::{baked, key::KeyHolder, nns_relay::Relay, tcp, tcp_client, OperatingMode};
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;
type SocketList = Arc<Mutex<HashMap<String, (TCPSocket, PeerKind)>>>;

type Connection = Arc<Mutex<Peer>>;

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
    let coordinator = {
        loop {
            match tcp_client::Peer::connect(baked::COORDINATOR_WELL_KNOWN, &nostr_client).await {
                Ok(connection) => break connection,
                Err(_) => {
                    println!(
                        "{}",
                        "Failed to connect to coordinator. Re-trying in 5..".red()
                    );
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };
        }
    };

    // uptimer
    {
        let coordinator = Arc::clone(&coordinator);
        tokio::spawn(async move {
            uptime(&coordinator).await;
        });
    }

    println!(
        "{}",
        "Enter command (type help for options, type exit to quit):".cyan()
    );

    cli(&coordinator).await;
}

pub async fn cli(coordinator_conn: &Connection) {
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
            _ => break,
        }
    }
}

fn handle_clear_command() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

async fn handle_conn_command(coordinator_conn: &Connection) {
    let _coordinator_conn = coordinator_conn.lock().await;

    _coordinator_conn.conn().await;
}

async fn handle_ping_command(coordinator_conn: &Connection) {
    let _coordinator_conn = coordinator_conn.lock().await;
    match _coordinator_conn.socket() {
        Some(socket) => match tcp_client::ping(&socket).await {
            Ok(_) => println!("Ponged."),
            Err(_) => println!("Error pinging."),
        },
        None => {
            println!("Coordinator connection dropped.");
        }
    }
}
