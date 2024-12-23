use crate::{key::KeyHolder, OperatingMode};

#[tokio::main]
pub async fn run(keys: KeyHolder, mode: OperatingMode) {
    let npub = keys.npub();

    println!("Running in operator mode with npub: {}", npub);
}
