use crate::Peer;

// conn
pub async fn command(coordinator: &Peer) {
    let _coordinator = coordinator.lock().await;

    match _coordinator.connection() {
        Some(_) => {
            let addr: String = _coordinator.addr();
            println!("Alive: {}", addr);
        }
        None => {
            println!("Dead.")
        }
    }
}
