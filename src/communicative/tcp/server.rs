use crate::tcp::RequestKind;
use crate::{baked, tcp, OperatingMode};
use colored::Colorize;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{io::AsyncWriteExt, net::TcpListener, sync::Mutex};

type TcpSocket = Arc<Mutex<tokio::net::TcpStream>>;
type ClientList = Arc<Mutex<HashMap<String, TcpSocket>>>;

const IDLE_TIMEOUT: Duration = Duration::from_secs(3600);

pub async fn run(client_list: &ClientList, mode: OperatingMode) {
    match mode {
        OperatingMode::Coordinator => (),
        OperatingMode::Operator => (),
        OperatingMode::Node => return, // Regular nodes do not run the server.
    }

    let addr = format!("{}:{}", "0.0.0.0", baked::PORT);

    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(_) => {
            eprintln!("{}", format!("Failed to bind {}.", addr).red());

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
                handle_socket(&socket, &client_list, &client_id, mode).await;
            }
        });
    }
}

async fn handle_socket(
    socket: &TcpSocket,
    client_list: &ClientList,
    client_id: &str,
    mode: OperatingMode,
) {
    loop {
        {
            let mut _socket = socket.lock().await;

            // Read requestcode.
            let mut requestcode_buffer = [0; 4];

            match tokio::time::timeout(
                IDLE_TIMEOUT,
                tcp::read(&mut *_socket, &mut requestcode_buffer),
            )
            .await
            {
                Ok(Ok(_)) => (),
                Ok(Err(tcp::TCPError::ConnErr)) => break, // Exit on disconnection.
                Ok(Err(_)) => continue,                   // Disregard by continuing.
                Err(_) => break,                          // Exit on idle timeout.
            }

            // Read payload length.
            let mut payload_length_buffer = [0; 4];

            match tokio::time::timeout(
                IDLE_TIMEOUT,
                tcp::read(&mut *_socket, &mut payload_length_buffer),
            )
            .await
            {
                Ok(Ok(_)) => (),
                Ok(Err(tcp::TCPError::ConnErr)) => break, // Exit on disconnection.
                Ok(Err(_)) => continue,                   // Disregard by continuing.
                Err(_) => break,                          // Exit on idle timeout.
            }

            let payload_length = u32::from_be_bytes(payload_length_buffer) as usize;

            // Read payload.
            let mut payload_buffer = vec![0; payload_length];

            match tokio::time::timeout(IDLE_TIMEOUT, tcp::read(&mut *_socket, &mut payload_buffer))
                .await
            {
                Ok(Ok(_)) => (),
                Ok(Err(tcp::TCPError::ConnErr)) => break, // Exit on disconnection.
                Ok(Err(_)) => continue,                   // Disregard by continuing.
                Err(_) => break,                          // Exit on idle timeout.
            }

            // Process the request kind.
            match RequestKind::from_requestcode(requestcode_buffer) {
                None => continue, // Skip invalid request kinds
                Some(kind) => handle_request(kind, &mut *_socket, &payload_buffer, mode).await,
            }
        }

        // For each iteration add a small delay after handling the socket.
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Remove the client from the list upon disconnection.
    {
        let mut _client_list = client_list.lock().await;
        _client_list.remove(client_id);
    }
}

async fn handle_request(
    kind: RequestKind,
    socket: &mut tokio::net::TcpStream,
    _payload: &[u8],
    mode: OperatingMode,
) {
    match mode {
        OperatingMode::Coordinator => match kind {
            RequestKind::Ping => handle_ping(socket, _payload).await,
            //_ => return,
        },
        OperatingMode::Operator => match kind {
            RequestKind::Ping => handle_ping(socket, _payload).await,
            //_ => return,
        },
        OperatingMode::Node => return,
    }
}

async fn handle_ping(socket: &mut tokio::net::TcpStream, _payload: &[u8]) {
    let response = RequestKind::Ping.to_requestcode();
    let response_len = (response.len() as u32).to_be_bytes();

    if let Err(_) = socket.write_all(&response_len).await {
        return;
    }

    if let Err(_) = socket.write_all(&response).await {
        return;
    }
}
