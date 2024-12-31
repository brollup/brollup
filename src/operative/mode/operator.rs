use crate::db;
use crate::nns_client;
use crate::tcp;
use crate::tcp_server;
use crate::vse;
use crate::Network;
use crate::OperatingMode;
use crate::SignatoryDB;
use crate::VSEDirectory;
use crate::{baked, key::KeyHolder, nns_server};
use colored::Colorize;
use std::io::{self, BufRead, Write};
use std::sync::Arc;
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
    let nns_client = nns_client::Client::new(&keys).await;

    // 2. Initialize signatory database.
    let signatory_db: SignatoryDB = match db::Signatory::new() {
        Some(database) => Arc::new(Mutex::new(database)),
        None => return eprintln!("{}", "Error initializing database.".red()),
    };

    // 3. Initialize VSE Directory.
    let vse_directory: VSEDirectory = match vse::Directory::new(&signatory_db).await {
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
            let _ = nns_server::run(&nns_client, mode).await;
        });
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

    // 8. CLI
    cli(&vse_directory).await;
}

pub async fn cli(vse_directory: &VSEDirectory) {
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
            "clear" => handle_clear_command(),
            "vse" => handle_vse_command(vse_directory, parts).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}

async fn handle_vse_command(vse_directory: &VSEDirectory, parts: Vec<&str>) {
    match parts.len() {
        2 => match parts[1].parse::<u64>() {
            Ok(no) => {
                let directory_ = {
                    let mut _vse_directory = vse_directory.lock().await;
                    (*_vse_directory).clone()
                };

                match directory_.setup(no) {
                    Some(setup) => {
                        setup.print();
                    }
                    None => eprintln!("VSE directory not available."),
                }
            }
            Err(_) => eprintln!("Invalid <no>."),
        },
        _ => {
            eprintln!("Invalid command.")
        }
    }
}

fn handle_clear_command() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}
