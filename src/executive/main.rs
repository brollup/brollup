use brollup::{
    coordinator,
    key::{FromNostrKeyStr, KeyHolder},
    node, operator, Network,
};
use colored::Colorize;
use std::{env, io::BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure at least 3 arguments: program name, network, mode.
    if args.len() < 3 {
        eprintln!("{}", format!("Usage: {} <network> <mode>", args[0]).red());
        return;
    }

    let network = &args[1];

    let network = match network.to_lowercase().as_str() {
        "signet" => Network::Signet,
        "mainnet" => Network::Mainnet,
        _ => {
            println!("{}", "Invalid <network>.".red());
            return;
        }
    };

    let mode = &args[2];

    let mut secret_key = [0xffu8; 32];

    println!("{}", "Enter nsec:".magenta());

    let stdin = std::io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.len() != 1 {
            println!("{}", "Invalid nsec.".yellow());
        }

        let nsec = parts[0];

        secret_key = match nsec.from_nsec() {
            Some(secret_key) => secret_key,
            None => {
                eprintln!("{}", "Invalid nsec.".red());
                return;
            }
        };

        break;
    }

    let keys = match KeyHolder::new(secret_key) {
        Some(key_holder) => key_holder,
        None => {
            eprintln!("{}", "Invalid nsec.".red());
            return;
        }
    };

    match mode.to_lowercase().as_str() {
        "node" => node::run(keys, network),
        "operator" => operator::run(keys, network),
        "coordinator" => coordinator::run(keys, network),
        _ => {
            println!("{}", "Invalid <mode>.".red());
            return;
        }
    }
}
