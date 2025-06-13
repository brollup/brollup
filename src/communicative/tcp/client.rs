use super::package::{PackageKind, TCPPackage};
use super::tcp::{self, TCPError};
use crate::communicative::peer::peer::{PeerConnection, PEER, SOCKET};
use crate::operative::session::commit::NSessionCommit;
use crate::operative::session::commitack::CSessionCommitAck;
use crate::operative::session::commitnack::CSessionCommitNack;
use crate::operative::session::opcov::CSessionOpCov;
use crate::operative::session::opcovack::OSessionOpCovAck;
use crate::operative::session::uphold::NSessionUphold;
use crate::operative::session::upholdack::CSessionUpholdAck;
use crate::operative::session::upholdnack::CSessionUpholdNack;
use crate::transmutative::musig::session::MusigSessionCtx;
use crate::transmutative::noist::dkg::package::DKGPackage;
use crate::transmutative::noist::dkg::session::DKGSession;
use crate::transmutative::noist::setup::keymap::VSEKeyMap;
use crate::transmutative::noist::setup::setup::VSESetup;
use crate::transmutative::secp::authenticable::Authenticable;
use async_trait::async_trait;
use chrono::Utc;
use secp::Scalar;
use std::collections::HashMap;
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

    async fn request_partial_sigs(
        &self,
        dir_height: u64,
        requests: &Vec<(u64, [u8; 32], Option<MusigSessionCtx>)>,
    ) -> Result<Vec<Scalar>, RequestError>;

    async fn sync_dkg_dir(
        &self,
        dir_height: u64,
    ) -> Result<(VSESetup, Vec<DKGSession>), RequestError>;

    /// Coordinator asking operators partial signatires given CSessionOpCov, returning OSessionOpCovAck.
    async fn request_opcov(&self, opcov: CSessionOpCov) -> Result<OSessionOpCovAck, RequestError>;

    /// msg.sender asking the coordinator to commit to a rollup state transition session.
    async fn commit_session(
        &self,
        auth_commit: Authenticable<NSessionCommit>,
    ) -> Result<Result<CSessionCommitAck, CSessionCommitNack>, RequestError>;

    /// msg.sender returning the coordinator to uphold a rollup state transition session.
    async fn uphold_session(
        &self,
        auth_uphold: Authenticable<NSessionUphold>,
    ) -> Result<Result<CSessionUpholdAck, CSessionUpholdNack>, RequestError>;
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
    /// Pinging.
    async fn ping(&self) -> Result<Duration, RequestError> {
        let payload = [0x00u8];

        // Build request package.
        let request_package = {
            let kind = PackageKind::Ping;
            let timestamp = Utc::now().timestamp();
            TCPPackage::new(kind, timestamp, &payload)
        };

        // Return the TCP socket.
        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        // Wait for the 'pong' for 3 seconds.
        let timeout = Duration::from_millis(3_000);

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

    /// This is during setup, the coordinator
    /// asking the operators their VSE keymap.
    async fn request_vse_keymap(
        &self,
        signatory_keys: &Vec<[u8; 32]>,
    ) -> Result<VSEKeyMap, RequestError> {
        let signatory_keys = signatory_keys.clone();

        let payload =
            serde_json::to_vec(&signatory_keys).map_err(|_| RequestError::InvalidRequest)?;

        // Build request package.
        let request_package = {
            let kind = PackageKind::RequestVSEKeymap;
            let timestamp = Utc::now().timestamp();
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

    /// This is during setup, the coordinator delivering VSE setup
    /// to the operators after collecting individual VSE keymaps from them.
    async fn deliver_vse_setup(&self, vse_setup: &VSESetup) -> Result<(), RequestError> {
        let payload = vse_setup.serialize();

        // Build request package.
        let request_package = {
            let kind = PackageKind::DeliverVSESetup;
            let timestamp = Utc::now().timestamp();
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

    /// This is during preprocessing, the coordinator requesting
    /// DKG package contributions from the operators.
    async fn request_dkg_packages(
        &self,
        setup_no: u64,
        package_count: u64,
    ) -> Result<Vec<Authenticable<DKGPackage>>, RequestError> {
        let payload = serde_json::to_vec(&(setup_no, package_count))
            .map_err(|_| RequestError::InvalidRequest)?;

        // Build request package.
        let request_package = {
            let kind = PackageKind::RequestDKGPackages;
            let timestamp = Utc::now().timestamp();
            TCPPackage::new(kind, timestamp, &payload)
        };

        // Return the TCP socket.
        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        // 1250ms base plus 10 ms for each requested package.
        let timeout = Duration::from_millis(1250 + package_count * 10);

        let (response_package, _) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        let auth_packages: Vec<Authenticable<DKGPackage>> =
            serde_json::from_slice(&response_payload).map_err(|_| RequestError::InvalidResponse)?;

        Ok(auth_packages)
    }

    /// This is during preprocessing, the coordinator relaying DKG sessions
    /// to the operators after collecting individual DKG packages from them.
    async fn deliver_dkg_sessions(
        &self,
        dir_height: u64,
        dkg_sessions: Vec<DKGSession>,
    ) -> Result<(), RequestError> {
        let dkg_sessions_len = dkg_sessions.len() as u64;

        let payload = serde_json::to_vec(&(dir_height, dkg_sessions))
            .map_err(|_| RequestError::InvalidRequest)?;

        // Build request package.
        let request_package = {
            let kind = PackageKind::DeliverDKGSessions;
            let timestamp = Utc::now().timestamp();
            TCPPackage::new(kind, timestamp, &payload)
        };

        // Return the TCP socket.
        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        // 1500ms base plus 10 ms for each requested package.
        let timeout = Duration::from_millis(1_500 + dkg_sessions_len * 10);

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

    /// This is during signing, the coordinator asking operators partial signatures
    /// for a given list of messages along with their nonce indexes.
    async fn request_partial_sigs(
        &self,
        dir_height: u64,
        requests: &Vec<(u64, [u8; 32], Option<MusigSessionCtx>)>,
    ) -> Result<Vec<Scalar>, RequestError> {
        let requests_len = requests.len() as u64;

        let payload = serde_json::to_vec(&(dir_height, requests.to_owned()))
            .map_err(|_| RequestError::InvalidRequest)?;

        // Build request package.
        let request_package = {
            let kind = PackageKind::RequestPartialSigs;
            let timestamp = Utc::now().timestamp();
            TCPPackage::new(kind, timestamp, &payload)
        };

        // Return the TCP socket.
        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        // 1000ms base plus 10 ms for each requested signature.
        let timeout = Duration::from_millis(1000 + requests_len * 10);

        let (response_package, _) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        let partial_sigs: Vec<Scalar> =
            serde_json::from_slice(&response_payload).map_err(|_| RequestError::InvalidResponse)?;

        Ok(partial_sigs)
    }

    /// This is the coordinator or an operator syncing with another peer
    /// the DKG directory in case they lost their local copy.
    async fn sync_dkg_dir(
        &self,
        dir_height: u64,
    ) -> Result<(VSESetup, Vec<DKGSession>), RequestError> {
        let payload = serde_json::to_vec(&dir_height).map_err(|_| RequestError::InvalidRequest)?;

        // Build request package.
        let request_package = {
            let kind = PackageKind::SyncDKGDir;
            let timestamp = Utc::now().timestamp();
            TCPPackage::new(kind, timestamp, &payload)
        };

        // Return the TCP socket.
        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        // Timeout 10 seconds.
        let timeout = Duration::from_millis(10_000);

        let (response_package, _) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        let (setup, sessions): (VSESetup, HashMap<u64, DKGSession>) =
            match serde_json::from_slice(&response_payload) {
                Ok(tuple) => tuple,
                Err(_) => return Err(RequestError::EmptyResponse),
            };

        let mut sorted_vec: Vec<(u64, DKGSession)> =
            sessions.into_iter().collect::<Vec<(u64, DKGSession)>>();

        sorted_vec.sort_by_key(|k| k.0);

        let sorted_sessions: Vec<DKGSession> =
            sorted_vec.into_iter().map(|(_, session)| session).collect();

        Ok((setup, sorted_sessions))
    }

    async fn request_opcov(&self, opcov: CSessionOpCov) -> Result<OSessionOpCovAck, RequestError> {
        let payload = serde_json::to_vec(&opcov).map_err(|_| RequestError::InvalidRequest)?;

        let request_package = {
            let kind = PackageKind::RequestOpCov;
            let timestamp = Utc::now().timestamp();
            TCPPackage::new(kind, timestamp, &payload)
        };

        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        let timeout = Duration::from_millis(1_000);

        let (response_package, _) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        let opcovack =
            serde_json::from_slice(&response_payload).map_err(|_| RequestError::InvalidResponse)?;

        Ok(opcovack)
    }

    async fn commit_session(
        &self,
        auth_commit: Authenticable<NSessionCommit>,
    ) -> Result<Result<CSessionCommitAck, CSessionCommitNack>, RequestError> {
        let payload = serde_json::to_vec(&auth_commit).map_err(|_| RequestError::InvalidRequest)?;

        let request_package = {
            let kind = PackageKind::CommitSession;
            let timestamp = Utc::now().timestamp();
            TCPPackage::new(kind, timestamp, &payload)
        };

        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        let timeout = Duration::from_millis(1_000);

        let (response_package, _) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        let commit_result: Result<CSessionCommitAck, CSessionCommitNack> =
            serde_json::from_slice(&response_payload).map_err(|_| RequestError::InvalidResponse)?;

        Ok(commit_result)
    }

    async fn uphold_session(
        &self,
        auth_uphold: Authenticable<NSessionUphold>,
    ) -> Result<Result<CSessionUpholdAck, CSessionUpholdNack>, RequestError> {
        let payload = serde_json::to_vec(&auth_uphold).map_err(|_| RequestError::InvalidRequest)?;

        let request_package = {
            let kind = PackageKind::UpholdSession;
            let timestamp = Utc::now().timestamp();
            TCPPackage::new(kind, timestamp, &payload)
        };

        let socket: SOCKET = self
            .socket()
            .await
            .ok_or(RequestError::TCPErr(TCPError::ConnErr))?;

        let timeout = Duration::from_millis(1_000);

        let (response_package, _) = tcp::request(&socket, request_package, Some(timeout))
            .await
            .map_err(|err| RequestError::TCPErr(err))?;

        let response_payload = match response_package.payload_len() {
            0 => return Err(RequestError::EmptyResponse),
            _ => response_package.payload(),
        };

        let uphold_result: Result<CSessionUpholdAck, CSessionUpholdNack> =
            serde_json::from_slice(&response_payload).map_err(|_| RequestError::InvalidResponse)?;

        Ok(uphold_result)
    }
}
