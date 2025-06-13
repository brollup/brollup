use crate::{
    inscriptive::{epoch::dir::EPOCH_DIRECTORY, wallet::wallet::WALLET},
    operative::Chain,
    transmutative::key::KeyHolder,
};

use super::addr::lift_address;

/// Prints the current set of lifts in the wallet.
pub async fn lift_command(
    wallet: &WALLET,
    epoch_dir: &EPOCH_DIRECTORY,
    chain: Chain,
    key_holder: &KeyHolder,
    parts: Vec<&str>,
) {
    match parts.get(1) {
        Some(part) => match part.to_owned() {
            "list" => lift_list(wallet).await,
            "addr" => lift_addr(chain, key_holder, epoch_dir).await,
            "up" => lift_up(wallet).await,
            _ => eprintln!("Unknown command."),
        },
        None => eprintln!("Incorrect usage."),
    }
}

async fn lift_addr(chain: Chain, key_holder: &KeyHolder, epoch_dir: &EPOCH_DIRECTORY) {
    let lift_address = match lift_address(chain, key_holder, epoch_dir).await {
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
        let lift_wallet = {
            let _wallet = wallet.lock().await;
            _wallet.lift_wallet()
        };

        let _lift_wallet = lift_wallet.lock().await;
        _lift_wallet.lifts()
    };

    // Create a vector of JSON values from each Lift
    let lift_jsons: Vec<serde_json::Value> = set.iter().map(|lift| lift.json()).collect();

    // Create a JSON array from the vector
    let json_array = serde_json::Value::Array(lift_jsons);

    // Print the JSON array
    match serde_json::to_string_pretty(&json_array) {
        Ok(json) => println!("{}", json),
        Err(_) => eprintln!("Error serializing lifts."),
    }
}
