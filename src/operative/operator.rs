use crate::key::KeyHolder;

#[tokio::main]
pub async fn run(keys: KeyHolder) {
    let npub = keys.npub();

    println!("Running in operator mode with npub: {}", npub);
}
