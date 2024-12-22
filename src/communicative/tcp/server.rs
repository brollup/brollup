use crate::baked;
use crate::tcp::RequestKind;
use colored::Colorize;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

const IDLE_CLIENT_PERIOD: Duration = Duration::from_secs(3600);

type TcpSocket = Arc<Mutex<tokio::net::TcpStream>>;
type ClientList = Arc<Mutex<HashMap<String, TcpSocket>>>;

pub async fn run(client_list: &ClientList) {
    let listener = match TcpListener::bind("0.0.0.0:".to_string() + &baked::PORT.to_string()).await
    {
        Ok(listener) => listener,
        Err(_) => {
            eprintln!("{}", "Failed to bind.".red());
            return;
        }
    };

    loop {
        let (socket_, socket_addr) = match listener.accept().await {
            Ok(conn) => (conn.0, conn.1),
            Err(_) => continue,
        };

        let socket = Arc::new(Mutex::new(socket_));
        let client_id = format!("{}:{}", socket_addr.ip(), socket_addr.port());

        {
            let mut _client_list = client_list.lock().await;
            _client_list.insert(client_id.clone(), Arc::clone(&socket));
        }

        tokio::spawn({
            let socket = Arc::clone(&socket);
            let client_list = Arc::clone(client_list);
            let client_id = client_id.clone();
            async move {
                handle_socket(&socket, &client_list, &client_id).await;
            }
        });
    }
}

async fn handle_socket(socket: &TcpSocket, client_list: &ClientList, client_id: &str) {
    loop {
        let mut _socket = socket.lock().await;

        let mut request_kind_buffer = [0; 1];
        let mut payload_length_buffer = [0; 4];

        // Read the request kind with a timeout
        match tokio::time::timeout(
            IDLE_CLIENT_PERIOD,
            _socket.read_exact(&mut request_kind_buffer),
        )
        .await
        {
            Ok(Ok(_)) => (),
            Ok(Err(inner_err)) => match inner_err.kind() {
                std::io::ErrorKind::UnexpectedEof => break, // Exit the loop on disconnection
                _ => continue,                              // Handle recoverable errors
            },
            Err(_) => break, // Exit the loop on inactivity.
        }

        // Read the payload length with a timeout
        match tokio::time::timeout(
            IDLE_CLIENT_PERIOD,
            _socket.read_exact(&mut payload_length_buffer),
        )
        .await
        {
            Ok(Ok(_)) => (),
            Ok(Err(inner_err)) => match inner_err.kind() {
                std::io::ErrorKind::UnexpectedEof => break, // Exit the loop on disconnection
                _ => continue,                              // Handle recoverable errors
            },
            Err(_) => break, // Exit the loop on inactivity.
        }

        let payload_length = u32::from_be_bytes(payload_length_buffer) as usize;
        let mut payload_buffer = vec![0; payload_length];

        // Read the payload with a timeout
        match tokio::time::timeout(IDLE_CLIENT_PERIOD, _socket.read_exact(&mut payload_buffer))
            .await
        {
            Ok(Ok(_)) => (),
            Ok(Err(inner_err)) => match inner_err.kind() {
                std::io::ErrorKind::UnexpectedEof => break, // Exit the loop on disconnection
                _ => continue,                              // Handle recoverable errors
            },
            Err(_) => break, // Exit the loop on inactivity.
        }

        // Process the request kind
        match RequestKind::from_bytecode(request_kind_buffer[0]) {
            None => {
                continue; // Skip invalid request kinds
            }
            Some(kind) => handle_request(kind, &mut _socket, &payload_buffer).await,
        }
    }

    // Remove the client from the client list
    let mut _client_list = client_list.lock().await;
    _client_list.remove(client_id);
}

async fn handle_request(kind: RequestKind, socket: &mut tokio::net::TcpStream, _payload: &[u8]) {
    match kind {
        RequestKind::Ping => {
            let response = vec![RequestKind::Ping.bytecode()];
            let response_len = (response.len() as u32).to_be_bytes();

            if let Err(_) = socket.write_all(&response_len).await {
                return;
            }

            if let Err(_) = socket.write_all(&response).await {
                return;
            }
        }
    }
}
