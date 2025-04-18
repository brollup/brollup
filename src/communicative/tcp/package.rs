use super::tcp::{self, TCPError};
use crate::communicative::peer::peer::SOCKET;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq)]
pub enum PackageKind {
    Ping,
    RequestVSEKeymap,
    DeliverVSESetup,
    RequestDKGPackages,
    DeliverDKGSessions,
    RequestPartialSigs,
    SyncDKGDir,
    RequestOpCov,
    CommitSession,
    UpholdSession,
}

impl PackageKind {
    pub fn bytecode(&self) -> u8 {
        match self {
            PackageKind::Ping => 0x00,
            PackageKind::RequestVSEKeymap => 0x01,
            PackageKind::DeliverVSESetup => 0x02,
            PackageKind::RequestDKGPackages => 0x03,
            PackageKind::DeliverDKGSessions => 0x04,
            PackageKind::RequestPartialSigs => 0x05,
            PackageKind::SyncDKGDir => 0x06,
            PackageKind::RequestOpCov => 0x07,
            PackageKind::CommitSession => 0x08,
            PackageKind::UpholdSession => 0x09,
        }
    }
    pub fn from_bytecode(bytecode: u8) -> Option<Self> {
        match bytecode {
            0x00 => Some(PackageKind::Ping),
            0x01 => Some(PackageKind::RequestVSEKeymap),
            0x02 => Some(PackageKind::DeliverVSESetup),
            0x03 => Some(PackageKind::RequestDKGPackages),
            0x04 => Some(PackageKind::DeliverDKGSessions),
            0x05 => Some(PackageKind::RequestPartialSigs),
            0x06 => Some(PackageKind::SyncDKGDir),
            0x07 => Some(PackageKind::RequestOpCov),
            0x08 => Some(PackageKind::CommitSession),
            0x09 => Some(PackageKind::UpholdSession),
            _ => None,
        }
    }
}

pub struct TCPPackage {
    kind: PackageKind,
    timestamp: i64,
    payload: Vec<u8>,
}

impl TCPPackage {
    pub fn new(kind: PackageKind, timestamp: i64, payload: &[u8]) -> TCPPackage {
        TCPPackage {
            kind,
            timestamp,
            payload: payload.to_vec(),
        }
    }

    pub fn kind(&self) -> PackageKind {
        self.kind
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn payload_len(&self) -> u32 {
        self.payload.len() as u32
    }

    pub fn payload(&self) -> Vec<u8> {
        self.payload.clone()
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        bytes.extend([self.kind().bytecode()]);
        bytes.extend(self.timestamp().to_be_bytes());
        bytes.extend(self.payload_len().to_be_bytes());
        bytes.extend(self.payload());

        bytes
    }

    pub async fn deliver(
        &self,
        socket: &SOCKET,
        timeout: Option<Duration>,
    ) -> Result<(), TCPError> {
        let mut _socket = socket.lock().await;
        tcp::write(&mut _socket, &self.serialize(), timeout).await
    }
}
