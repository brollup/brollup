use crate::{tcp::client::TCPClient, PEER};

// ping
pub async fn ping_command(coordinator: &PEER) {
    match coordinator.ping().await {
        Ok(duration) => println!("{} ms", duration.as_millis()),
        Err(_) => println!("Error pinging."),
    }
}
