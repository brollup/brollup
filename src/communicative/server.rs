use crate::key::ToNostrKeyStr;
use crate::tcp_request::RequestKind;
use crate::{baked, nns_query, tcp, OperatingMode};
use colored::Colorize;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::Instant;
use tokio::{io::AsyncWriteExt, net::TcpListener, sync::Mutex};

type TCPSocket = Arc<Mutex<tokio::net::TcpStream>>;
type ClientList = Arc<Mutex<HashMap<String, TCPSocket>>>;
type NostrClient = Arc<Mutex<nostr_sdk::Client>>;

pub const IDLE_CLIENT_TIMEOUT: Duration = Duration::from_secs(60);
pub const SOCKET_HANDLE_TIMEOUT: Duration = Duration::from_millis(1500);

pub async fn run(client_list: &ClientList, mode: OperatingMode, nostr_client: &NostrClient) {
    let addr = format!("{}:{}", "0.0.0.0", baked::PORT);

    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(_) => {
            eprintln!("{}", format!("Failed to bind {}.", addr).red());

            return;
        }
    };

    match mode {
        OperatingMode::Coordinator => loop {
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
                    handle_socket(&socket, &client_id, &client_list, mode).await;
                }
            });
        },
        OperatingMode::Operator => {
            let coordinator_npub = match baked::COORDINATOR_WELL_KNOWN.to_npub() {
                Some(npub) => npub,
                None => return,
            };

            loop {
                match nns_query::address(&coordinator_npub, nostr_client).await {
                    Some(ip_address) => loop {
                        let (socket_, socket_addr) = match listener.accept().await {
                            Ok(conn) => (conn.0, conn.1),
                            Err(_) => continue,
                        };

                        // Operator only accepts incoming connections from the coordinator.
                        if socket_addr.ip().to_string() != ip_address {
                            continue;
                        }

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
                                handle_socket(&socket, &client_id, &client_list, mode).await;
                            }
                        });
                    },
                    None => {
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                }
            }
        }
        OperatingMode::Node => return,
    }
}

async fn handle_socket(
    socket: &TCPSocket,
    client_id: &str,
    client_list: &ClientList,
    mode: OperatingMode,
) {
    loop {
        {
            let mut _socket = socket.lock().await;

            // Read requestcode.
            let mut requestcode = [0; 4];
            match tcp::read(&mut *_socket, &mut requestcode, Some(IDLE_CLIENT_TIMEOUT)).await {
                Ok(_) => (),
                Err(tcp::TCPError::ConnErr) => break, // Exit on disconnection.
                Err(tcp::TCPError::Timeout) => break, // Exit on IDLE_TIMEOUT.
                Err(_) => continue,                   // Iterate on read errors.
            }

            // Start tracking elapsed time.
            let start = Instant::now();
            let timeout_duration = SOCKET_HANDLE_TIMEOUT; // Default timeout: 1500 ms.

            // Read payload length.
            let mut payload_len = [0; 4];
            match tcp::read(&mut *_socket, &mut payload_len, Some(timeout_duration)).await {
                Ok(_) => (),
                Err(tcp::TCPError::ConnErr) => break, // Exit on disconnection.
                Err(tcp::TCPError::Timeout) => continue, // Iterate on REQUEST_TIMEOUT.
                Err(_) => continue,                   // Iterate on read errors.
            }

            let remaining_time = match timeout_duration.checked_sub(start.elapsed()) {
                Some(duration) => duration,
                None => continue,
            };

            // Read payload.
            let mut payload = vec![0; u32::from_be_bytes(payload_len) as usize];
            match tcp::read(&mut *_socket, &mut payload, Some(remaining_time)).await {
                Ok(_) => (),
                Err(tcp::TCPError::ConnErr) => break, // Exit on disconnection.
                Err(tcp::TCPError::Timeout) => continue, // Iterate on REQUEST_TIMEOUT.
                Err(_) => continue,                   // Iterate on read errors.
            }

            // Process the request kind.
            match RequestKind::from_requestcode(requestcode) {
                None => continue, // Skip invalid request kinds
                Some(kind) => handle_request(kind, &mut *_socket, &payload, mode).await,
            }
        }

        // For each iteration add a small delay after handling the socket.
        tokio::time::sleep(Duration::from_millis(10)).await;
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

async fn handle_ping(socket: &mut tokio::net::TcpStream, payload: &[u8]) {
    let response = RequestKind::Ping.to_requestcode(); // Pong.
    let response_len = (response.len() as u32).to_be_bytes();

    if payload == &[0x00] {
        if let Err(_) = socket.write_all(&response_len).await {
            return;
        }

        if let Err(_) = socket.write_all(&response).await {
            return;
        }
    } else {
        return;
    }
}
