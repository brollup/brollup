#![allow(non_camel_case_types)]

use std::sync::Arc;
use tokio::sync::Mutex;

// Signatory.
type SIGNATORY_DB = Arc<Mutex<db::Signatory>>;
type VSE_DIRECTORY = Arc<Mutex<vse::Directory>>;
type VSE_SETUP = Arc<Mutex<vse::Setup>>;

// Networking.
type SOCKET = Arc<Mutex<tokio::net::TcpStream>>;
type PEER = Arc<Mutex<tcp_peer::Peer>>;
type PEER_LIST = Arc<Mutex<Vec<PEER>>>;

#[path = "constructive/list.rs"]
pub mod list;

// Protocol
#[path = "operative/protocol/vse_setup.rs"]
pub mod vse_setup;

// Inscriptive
#[path = "inscriptive/db.rs"]
pub mod db;

#[path = "inscriptive/baked.rs"]
pub mod baked;

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
#[path = "operative/mode/node/node.rs"]
pub mod node;

#[path = "operative/mode/operator/operator.rs"]
pub mod operator;

#[path = "operative/mode/coordinator/coordinator.rs"]
pub mod coordinator;

// CLI
// Operating modes.
#[path = "operative/mode/node/cli/mod.rs"]
pub mod ncli;

#[path = "operative/mode/operator/cli/mod.rs"]
pub mod ocli;

#[path = "operative/mode/coordinator/cli/mod.rs"]
pub mod ccli;

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

#[path = "communicative/tcp/peer.rs"]
pub mod tcp_peer;

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
