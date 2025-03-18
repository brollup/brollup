use crate::{key::KeyHolder, Network, EPOCH_DIRECTORY, WALLET};

use super::addr::lift_address;

/// Prints the current set of lifts in the wallet.
pub async fn lift_command(
    wallet: &WALLET,
    epoch_dir: &EPOCH_DIRECTORY,
    network: Network,
    key_holder: &KeyHolder,
    parts: Vec<&str>,
) {
    match parts.get(1) {
        Some(part) => match part.to_owned() {
            "list" => lift_list(wallet).await,
            "addr" => lift_addr(network, key_holder, epoch_dir).await,
            "up" => lift_up(wallet).await,
            _ => eprintln!("Unknown command."),
        },
        None => eprintln!("Incorrect usage."),
    }
}

async fn lift_addr(network: Network, key_holder: &KeyHolder, epoch_dir: &EPOCH_DIRECTORY) {
    let lift_address = match lift_address(network, key_holder, epoch_dir).await {
        Some(address) => address,
        None => "-".to_string(),
    };

    println!("{}", lift_address);
}

async fn lift_up(_wallet: &WALLET) {
    // TODO: Implement
}

async fn lift_list(wallet: &WALLET) {
    let set = {
        let _wallet = wallet.lock().await;
        _wallet.lifts()
    };

    match serde_json::to_string_pretty(&set) {
        Ok(json) => println!("{}", json),
        Err(_) => eprintln!("Error serializing lifts."),
    }
}
