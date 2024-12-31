use crate::{baked, key::KeyHolder, tcp_server};
use crate::{
    ccli, db, nns_client, tcp, tcp_client, vse, Network, OperatingMode, Peer, PeerList,
    SignatoryDB, VSEDirectory,
};
use colored::Colorize;

use std::io::{self, BufRead};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
pub async fn run(keys: KeyHolder, _network: Network) {
    let mode = OperatingMode::Coordinator;

    if keys.public_key() != baked::COORDINATOR_WELL_KNOWN {
        eprintln!("{}", "Coordinator <nsec> does not match.".red());
        return;
    }

    println!("{}", "Initializing coordinator..");

    // 1. Initialize NNS client.
    let nns_client = nns_client::Client::new(&keys).await;

    // 2. Initialize signatory database.
    let signatory_db: SignatoryDB = match db::Signatory::new() {
        Some(database) => Arc::new(Mutex::new(database)),
        None => return eprintln!("{}", "Error initializing database.".red()),
    };

    // 3. Initialize VSE Directory.
    let mut vse_directory: VSEDirectory = match vse::Directory::new(&signatory_db).await {
        Some(directory) => Arc::new(Mutex::new(directory)),
        None => return eprintln!("{}", "Error initializing VSE directory.".red()),
    };

    // 4. Open port 6272 for incoming connections.
    match tcp::open_port().await {
        true => println!("{}", format!("Opened port '{}'.", baked::PORT).green()),
        false => (),
    }

    // 6. Run TCP server.
    {
        let nns_client = nns_client.clone();
        let signatory_db = Arc::clone(&signatory_db);
        let vse_directory = Arc::clone(&vse_directory);

        let _ = tokio::spawn(async move {
            let _ = tcp_server::run(mode, &nns_client, &keys, &signatory_db, &vse_directory).await;
        });
    }

    // 7. Initialize operator list.
    let operator_list: PeerList = Arc::new(Mutex::new(Vec::<Peer>::new()));

    // 8. Connect to operators.
    for nns_key in baked::OPERATOR_SET.iter() {
        let nns_client = nns_client.clone();
        let operator_list = Arc::clone(&operator_list);

        tokio::spawn(async move {
            let operator: Peer = loop {
                match tcp_client::Peer::connect(
                    tcp_client::PeerKind::Operator,
                    nns_key.to_owned(),
                    &nns_client,
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

    // 9. CLI
    cli(&operator_list, &signatory_db, &mut vse_directory).await;
}

pub async fn cli(
    operator_list: &PeerList,
    signatory_db: &SignatoryDB,
    vse_directory: &mut VSEDirectory,
) {
    println!(
        "{}",
        "Enter command (type help for options, type exit to quit):".cyan()
    );

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
            "clear" => ccli::clear::command(),
            "vse" => ccli::vse::command(parts, operator_list, signatory_db, vse_directory).await,
            "operator" => ccli::operator::command(operator_list).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
