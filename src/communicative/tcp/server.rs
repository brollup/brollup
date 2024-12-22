use crate::baked;
use crate::tcp::RequestKind;
use colored::Colorize;
use std::{collections::HashSet, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

/// Executes a persistent server task that accepts
/// incoming connections and handles them accordingly.
///
pub async fn run() {
    let listener = match TcpListener::bind("0.0.0.0:".to_string() + &baked::PORT.to_string()).await
    {
        Ok(listener) => listener,
        Err(_) => {
            println!("{}", "Failed to bind.".red());
            return;
        }
    };

    let client_ips = Arc::new(Mutex::new(HashSet::new()));

    loop {
        let (socket, addr) = match listener.accept().await {
            Ok(conn) => conn,
            Err(_) => continue,
        };

        {
            let mut ips = client_ips.lock().await;
            ips.insert(addr.ip());
        }

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut request_kind_buffer = [0; 1];
        let mut payload_length_buffer = [0; 4];

        if stream.read_exact(&mut request_kind_buffer).await.is_err() {
            return;
        }

        if stream.read_exact(&mut payload_length_buffer).await.is_err() {
            return;
        }

        let payload_length = u32::from_be_bytes(payload_length_buffer) as usize;

        let mut payload_buffer = vec![0; payload_length];
        if stream.read_exact(&mut payload_buffer).await.is_err() {
            return;
        }

        match RequestKind::from_bytecode(request_kind_buffer[0]) {
            None => (),
            Some(kind) => match kind {
                RequestKind::Ping => {
                    let response = vec![RequestKind::Ping.bytecode()];
                    let response_len = (response.len() as u32).to_be_bytes();

                    if stream.write_all(&response_len).await.is_err() {
                        return;
                    }

                    if stream.write_all(&response).await.is_err() {
                        return;
                    }
                }
            },
        }
    }
}
