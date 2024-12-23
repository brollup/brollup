use std::{env, io::BufRead};

use brollup::{
    coordinator,
    key::{FromNostrKeyStr, KeyHolder},
    node, operator, OperatingMode,
};
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure at least 3 arguments: program name, mode.
    if args.len() < 2 {
        eprintln!("{}", format!("Usage: {} <mode>", args[0]).red());
        return;
    }

    let mode = &args[1];

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
                eprintln!("{}", "Invalid <nsec>.".red());
                return;
            }
        };

        break;
    }

    let keys = match KeyHolder::new(secret_key) {
        Some(key_holder) => key_holder,
        None => {
            eprintln!("{}", "Invalid <nsec>.".red());
            return;
        }
    };

    match mode.as_str() {
        "node" => node::run(keys, OperatingMode::Node),
        "operator" => operator::run(keys, OperatingMode::Operator),
        "coordinator" => coordinator::run(keys, OperatingMode::Coordinator),
        _ => {
            eprintln!("Error: Unknown mode '{}'", mode);
            eprintln!("Valid modes are: node, operator, coordinator");
        }
    }
}
