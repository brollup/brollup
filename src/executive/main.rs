use std::env;

use brollup::{
    coordinator,
    key::{FromNostrKeyStr, KeyHolder},
    node, operator,
};
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure at least 3 arguments: program name, mode, and <nsec>
    if args.len() < 3 {
        eprintln!("{}", format!("Usage: {} <mode> <nsec>", args[0]).red());
        return;
    }

    let mode = &args[1];
    let nsec = &args[2];

    let secret_key = match nsec.as_str().from_nsec() {
        Some(secret_key) => secret_key,
        None => {
            eprintln!("{}", "Invalid <nsec>.".red());
            return;
        }
    };

    let keys = match KeyHolder::new(secret_key) {
        Some(key_holder) => key_holder,
        None => {
            eprintln!("{}", "Invalid <nsec>.".red());
            return;
        }
    };

    match mode.as_str() {
        "node" => node::run(keys),
        "operator" => operator::run(keys),
        "coordinator" => coordinator::run(keys),
        _ => {
            eprintln!("Error: Unknown mode '{}'", mode);
            eprintln!("Valid modes are: node, operator, coordinator");
        }
    }
}
