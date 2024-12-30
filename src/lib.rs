use std::sync::Arc;
use tokio::sync::Mutex;

// Signatory.
type SignatoryDB = Arc<Mutex<db::Signatory>>;
type VSEDirectory = Arc<Mutex<vse::Directory>>;

// Networking.
type Socket = Arc<Mutex<tokio::net::TcpStream>>;
type Peer = Arc<Mutex<tcp_client::Peer>>;
type PeerList = Arc<Mutex<Vec<Peer>>>;

pub mod baked;

#[path = "constructive/list.rs"]
pub mod list;

// Protocol
#[path = "operative/protocol/vse_setup.rs"]
pub mod vse_setup_protocol;

// Inscriptive
#[path = "inscriptive/db.rs"]
pub mod db;

// Crypto modules.

#[path = "transmutive/into.rs"]
pub mod into;

#[path = "transmutive/hash.rs"]
pub mod hash;

#[path = "transmutive/key.rs"]
pub mod key;

#[path = "transmutive/schnorr.rs"]
pub mod schnorr;

#[path = "transmutive/noist/vse.rs"]
pub mod vse;

// Operating modes.
#[path = "operative/mode/node.rs"]
pub mod node;

#[path = "operative/mode/operator.rs"]
pub mod operator;

#[path = "operative/mode/coordinator.rs"]
pub mod coordinator;

// Networking.
#[path = "communicative/nns/server.rs"]
pub mod nns_server;

#[path = "communicative/nns/relay.rs"]
pub mod nns_relay;

#[path = "communicative/nns/client.rs"]
pub mod nns_client;

#[path = "communicative/tcp/tcp.rs"]
pub mod tcp;

#[path = "communicative/tcp/server.rs"]
pub mod tcp_server;

#[path = "communicative/tcp/client.rs"]
pub mod tcp_client;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Network {
    Signet,
    Mainnet,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperatingMode {
    Coordinator,
    Operator,
    Node,
}
