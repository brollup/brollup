use crate::{tcp_client::Request, Peer};

// ping
pub async fn command(coordinator: &Peer) {
    match coordinator.ping().await {
        Ok(duration) => println!("{} ms", duration.as_millis()),
        Err(_) => println!("Error pinging."),
    }
}
