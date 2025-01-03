use super::package::{PackageKind, TCPPackage};
use super::peer::Connection;
use super::tcp::{self, TCPError};
use crate::list::ListCodec;
use crate::noist::vse::{directory::VSEDirectory, keymap::VSEKeyMap, setup::VSESetup};
use crate::schnorr::Authenticable;

use crate::{PEER, SOCKET};
use async_trait::async_trait;
use chrono::Utc;
use std::time::Duration;

#[async_trait]
pub trait TCPClient {
    async fn ping(&self) -> Result<Duration, RequestError>;

    // Signatory requests.
    async fn request_vse_keymap(
        &self,
        signer_key: [u8; 32],
        signer_list: &Vec<[u8; 32]>,
    ) -> Result<Authenticable<VSEKeyMap>, RequestError>;

    async fn deliver_vse_setup(&self, vse_setup: &VSESetup) -> Result<(), RequestError>;

    async fn retrieve_vse_directory(&self) -> Result<VSEDirectory, RequestError>;
}

#[derive(Copy, Clone)]
pub enum RequestError {
    TCPErr(TCPError),
    InvalidResponse,
    EmptyResponse,
    ErrorResponse,
}

#[async_trait]
impl TCPClient for PEER {
    async fn ping(&self) -> Result<Duration, RequestError> {
        // Build request package.
        let request_package = {
            let kind = PackageKind::Ping;
            let timestamp = Utc::now().timestamp();
            let payload = [0x00u8];
            TCPPackage::new(kind, timestamp, &payload)
        };

        // Return the TCP socket.
        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        // Wait for the 'pong' for 10 seconds.
        let timeout = Duration::from_millis(10_000);

        let (response_package, duration) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        // Expected response: 0x01 for pong.
        if response_payload != [0x01u8] {
            return Err(RequestError::InvalidResponse);
        }

        Ok(duration)
    }

    // This is when the coordinator asks each operators to return their vse keymaps.
    async fn request_vse_keymap(
        &self,
        signer_key: [u8; 32],
        signer_list: &Vec<[u8; 32]>,
    ) -> Result<Authenticable<VSEKeyMap>, RequestError> {
        // Build request package.
        let request_package = {
            let kind = PackageKind::RequestVSEKeymap;
            let timestamp = Utc::now().timestamp();
            let payload = signer_list.encode_list();
            TCPPackage::new(kind, timestamp, &payload)
        };

        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        // Timeout 3 seconds.
        let timeout = Duration::from_millis(3_000);

        let (response_package, _) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        let auth_keymap: Authenticable<VSEKeyMap> =
            bincode::deserialize(&response_payload).map_err(|_| RequestError::InvalidResponse)?;

        println!("authen :{}", auth_keymap.authenticate());

        if (auth_keymap.key() != signer_key) || !auth_keymap.authenticate() {
            return Err(RequestError::InvalidResponse);
        }

        Ok(auth_keymap)
    }

    // This is when the coordinator publishes each operator the new vse directory.
    // Likely after retrieve_vse_keymap.
    async fn deliver_vse_setup(&self, vse_setup: &VSESetup) -> Result<(), RequestError> {
        // Build request package.
        let request_package = {
            let kind = PackageKind::DeliverVSESetup;
            let timestamp = Utc::now().timestamp();
            let payload = vse_setup.serialize();
            TCPPackage::new(kind, timestamp, &payload)
        };

        // Return the TCP socket.
        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        // Timeout 3 seconds.
        let timeout = Duration::from_millis(3_000);

        let (response_package, _) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        match response_payload.as_slice() {
            [0x01u8] => return Ok(()),
            [0x00u8] => return Err(RequestError::ErrorResponse),
            _ => return Err(RequestError::InvalidResponse),
        }
    }

    // This is a coordinator or the operator asking from another peer
    // about the vse directory in case they lost their local copy.
    async fn retrieve_vse_directory(&self) -> Result<VSEDirectory, RequestError> {
        // Build request package.
        let request_package = {
            let kind = PackageKind::RetrieveVSEDirectory;
            let timestamp = Utc::now().timestamp();
            let payload = [0x00u8];
            TCPPackage::new(kind, timestamp, &payload)
        };

        // Return the TCP socket.
        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        // Timeout 3 seconds.
        let timeout = Duration::from_millis(3_000);

        let (response_package, _) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        let vse_directory: VSEDirectory = match bincode::deserialize(&response_payload) {
            Ok(directory) => directory,
            Err(_) => return Err(RequestError::EmptyResponse),
        };

        Ok(vse_directory)
    }
}
