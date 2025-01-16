use super::package::{PackageKind, TCPPackage};
use super::tcp::{self, TCPError};
use crate::list::{self, ListCodec};
use crate::noist::dkg::package::DKGPackage;
use crate::noist::dkg::session::DKGSession;
use crate::noist::setup::{keymap::VSEKeyMap, setup::VSESetup};
use crate::peer::PeerConnection;
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
        signatory_keys: &Vec<[u8; 32]>,
    ) -> Result<VSEKeyMap, RequestError>;

    async fn deliver_vse_setup(&self, vse_setup: &VSESetup) -> Result<(), RequestError>;

    async fn retrieve_vse_setup(&self, setup_no: u64) -> Result<VSESetup, RequestError>;

    async fn request_dkg_packages(
        &self,
        setup_no: u64,
        count: u64,
    ) -> Result<Vec<Authenticable<DKGPackage>>, RequestError>;

    async fn deliver_dkg_sessions(
        &self,
        dir_height: u64,
        dkg_sessions: Vec<DKGSession>,
    ) -> Result<(), RequestError>;
}

#[derive(Copy, Clone)]
pub enum RequestError {
    TCPErr(TCPError),
    InvalidRequest,
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
        signatory_keys: &Vec<[u8; 32]>,
    ) -> Result<VSEKeyMap, RequestError> {
        // Build request package.
        let request_package = {
            let kind = PackageKind::RequestVSEKeymap;
            let timestamp = Utc::now().timestamp();
            let payload = signatory_keys.encode_list();
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

        let keymap = match VSEKeyMap::from_slice(&response_payload) {
            Some(keymap) => keymap,
            None => return Err(RequestError::InvalidResponse),
        };

        Ok(keymap)
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
    // about the vse setup in case they lost their local copy.
    async fn retrieve_vse_setup(&self, setup_no: u64) -> Result<VSESetup, RequestError> {
        // Build request package.
        let request_package = {
            let kind = PackageKind::RetrieveVSESetup;
            let timestamp = Utc::now().timestamp();
            let payload = setup_no.to_be_bytes();
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

        let vse_setup: VSESetup = match serde_json::from_slice(&response_payload) {
            Ok(setup) => setup,
            Err(_) => return Err(RequestError::EmptyResponse),
        };

        Ok(vse_setup)
    }

    // This is coordinator requesting operators new auth DKG packages.
    async fn request_dkg_packages(
        &self,
        setup_no: u64,
        count: u64,
    ) -> Result<Vec<Authenticable<DKGPackage>>, RequestError> {
        let setup_no_bytes = setup_no.to_be_bytes();
        let count_bytes = count.to_be_bytes();

        let mut payload = Vec::<u8>::with_capacity(16);
        payload.extend(setup_no_bytes);
        payload.extend(count_bytes);

        // Build request package.
        let request_package = {
            let kind = PackageKind::RequestDKGPackages;
            let timestamp = Utc::now().timestamp();
            let payload = setup_no.to_be_bytes();
            TCPPackage::new(kind, timestamp, &payload)
        };

        // Return the TCP socket.
        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        // 3 seconds base plus 10 ms for each requested package.
        let timeout = Duration::from_millis(3_000 + count * 10);

        let (response_package, _) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        let package_bytes: Vec<Vec<u8>> = match list::ListCodec::decode_list(&response_payload) {
            Some(vec) => vec,
            None => return Err(RequestError::InvalidResponse),
        };

        if package_bytes.len() != count as usize {
            return Err(RequestError::InvalidResponse);
        }

        let mut auth_dkg_packages = Vec::<Authenticable<DKGPackage>>::new();

        for bytes in package_bytes {
            let auth_dkg_package: Authenticable<DKGPackage> =
                serde_json::from_slice(&bytes).map_err(|_| RequestError::InvalidResponse)?;
            auth_dkg_packages.push(auth_dkg_package);
        }

        Ok(auth_dkg_packages)
    }

    async fn deliver_dkg_sessions(
        &self,
        dir_height: u64,
        dkg_sessions: Vec<DKGSession>,
    ) -> Result<(), RequestError> {
        let dir_height_bytes = dir_height.to_be_bytes();

        let dkg_sessions_bytes =
            serde_json::to_vec(&dkg_sessions).map_err(|_| RequestError::InvalidRequest)?;

        let mut payload = Vec::<u8>::with_capacity(8 + dkg_sessions_bytes.len());
        payload.extend(dir_height_bytes);
        payload.extend(dkg_sessions_bytes);

        // Build request package.
        let request_package = {
            let kind = PackageKind::DeliverDKGSessions;
            let timestamp = Utc::now().timestamp();
            let payload = payload;
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
}
