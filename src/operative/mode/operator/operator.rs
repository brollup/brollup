use crate::db;
use crate::nns;
use crate::nns::client::NNSClient;
use crate::noist::setup::directory::VSEDirectory;
use crate::ocli;
use crate::tcp::peer::Peer;
use crate::tcp::peer::PeerKind;
use crate::tcp::server;
use crate::tcp::tcp;
use crate::Network;
use crate::OperatingMode;
use crate::PEER;
use crate::SIGNATORY_DB;
use crate::VSE_DIRECTORY;
use crate::{baked, key::KeyHolder};
use colored::Colorize;
use std::io::{self, BufRead};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
pub async fn run(keys: KeyHolder, _network: Network) {
    let mode = OperatingMode::Operator;

    if !baked::OPERATOR_SET.contains(&keys.public_key()) {
        eprintln!("{}", "Operator <nsec> does not match.".red());
        return;
    }

    println!("{}", "Initializing operator..");

    // 1. Initialize NNS client.
    let nns_client = NNSClient::new(&keys).await;

    // 2. Initialize signatory database.
    let signatory_db: SIGNATORY_DB = match db::Signatory::new() {
        Some(database) => Arc::new(Mutex::new(database)),
        None => return eprintln!("{}", "Error initializing database.".red()),
    };

    // 3. Initialize VSE Directory.
    let mut vse_directory: VSE_DIRECTORY = match VSEDirectory::new(&signatory_db).await {
        Some(directory) => Arc::new(Mutex::new(directory)),
        None => return eprintln!("{}", "Error initializing VSE directory.".red()),
    };

    // 4. Open port 6272 for incoming connections.
    match tcp::open_port().await {
        true => println!("{}", format!("Opened port '{}'.", baked::PORT).green()),
        false => (),
    }

    // 5. Run NNS server.
    {
        let nns_client = nns_client.clone();
        let _ = tokio::spawn(async move {
            let _ = nns::server::run(&nns_client, mode).await;
        });
    }

    // 6. Run TCP server.
    {
        let nns_client = nns_client.clone();
        let signatory_db = Arc::clone(&signatory_db);
        let vse_directory = Arc::clone(&vse_directory);

        let _ = tokio::spawn(async move {
            let _ = server::run(mode, &nns_client, &keys, &signatory_db, &vse_directory).await;
        });
    }

    // 7. Connect to the coordinator.
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
                    println!(
                        "{}",
                        "Failed to connect coordinator. Re-trying in 5..".red()
                    );
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };
        }
    };

    // 8. CLI
    cli(&signatory_db, &mut vse_directory, &coordinator).await;
}

pub async fn cli(
    signatory_db: &SIGNATORY_DB,
    vse_directory: &mut VSE_DIRECTORY,
    coordinator: &PEER,
) {
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
            "clear" => ocli::clear::command(),
            "vse" => ocli::vse::command(parts, coordinator, signatory_db, vse_directory).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
