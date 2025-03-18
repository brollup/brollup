use crate::PEER;

// conn
pub async fn conn_command(coordinator: &PEER) {
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
