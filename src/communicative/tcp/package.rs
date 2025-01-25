use std::time::Duration;

use tokio::net::TcpStream;

use super::tcp::{self, TCPError};

#[derive(Copy, Clone, PartialEq)]
pub enum PackageKind {
    Ping,
    RequestVSEKeymap,
    DeliverVSESetup,
    RequestDKGPackages,
    DeliverDKGSessions,
    RequestPartialSigs,
    SyncDKGDir,
    CovSessionJoin,
    CovSessionSubmit,
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
            PackageKind::CovSessionJoin => 0x07,
            PackageKind::CovSessionSubmit => 0x08,
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
            0x07 => Some(PackageKind::CovSessionJoin),
            0x08 => Some(PackageKind::CovSessionSubmit),
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
        socket: &mut TcpStream,
        timeout: Option<Duration>,
    ) -> Result<(), TCPError> {
        tcp::write(socket, &self.serialize(), timeout).await
    }
}
