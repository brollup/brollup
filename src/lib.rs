#![allow(non_camel_case_types)]

use std::sync::Arc;
use tokio::sync::Mutex;

// Signatory.
type SIGNATORY_DB = Arc<Mutex<db::Signatory>>;
type VSE_DIRECTORY = Arc<Mutex<noist::setup::directory::VSEDirectory>>;
type VSE_SETUP = Arc<Mutex<noist::setup::setup::VSESetup>>;

// Networking.
type SOCKET = Arc<Mutex<tokio::net::TcpStream>>;
type PEER = Arc<Mutex<tcp::peer::Peer>>;
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
#[path = "transmutive/hash.rs"]
pub mod hash;
#[path = "transmutive/into.rs"]
pub mod into;
#[path = "transmutive/key.rs"]
pub mod key;
#[path = "transmutive/schnorr.rs"]
pub mod schnorr;

// NOIST.
#[path = "transmutive/noist/mod.rs"]
pub mod noist;

// Operating modes.
#[path = "operative/mode/coordinator/coordinator.rs"]
pub mod coordinator;
#[path = "operative/mode/node/node.rs"]
pub mod node;
#[path = "operative/mode/operator/operator.rs"]
pub mod operator;

// Command line.
#[path = "operative/mode/coordinator/cli/mod.rs"]
pub mod ccli;
#[path = "operative/mode/node/cli/mod.rs"]
pub mod ncli;
#[path = "operative/mode/operator/cli/mod.rs"]
pub mod ocli;

// Networking.
#[path = "communicative/nns/mod.rs"]
pub mod nns;
#[path = "communicative/tcp/mod.rs"]
pub mod tcp;

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
