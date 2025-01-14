#![allow(non_camel_case_types)]

use noist::{
    dkg::{directory::DKGDirectory, session::DKGSession},
    manager::DKGManager,
};
use std::sync::Arc;
use tokio::sync::Mutex;

// Networking.
type SOCKET = Arc<Mutex<tokio::net::TcpStream>>;
type PEER = Arc<Mutex<peer::Peer>>;
type PEER_MANAGER = Arc<Mutex<peer_manager::PeerManager>>;
type DKG_MANAGER = Arc<Mutex<DKGManager>>;
type DKG_DIRECTORY = Arc<Mutex<DKGDirectory>>;
type DKG_SESSION = Arc<Mutex<DKGSession>>;

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
#[path = "transmutive/point.rs"]
pub mod secp_point;

#[path = "transmutive/list.rs"]
pub mod list;

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

// Liquidity provision
#[path = "operative/liquidity/mod.rs"]
pub mod liquidity;

// Signatory
#[path = "operative/signatory/mod.rs"]
pub mod signatory;

// Networking.
#[path = "communicative/nns/mod.rs"]
pub mod nns;
#[path = "communicative/peer/peer.rs"]
pub mod peer;
#[path = "communicative/peer/manager.rs"]
pub mod peer_manager;
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
