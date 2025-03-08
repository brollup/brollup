#![allow(non_camel_case_types)]

use blaming::BlamingDirectory;
use noist::{
    dkg::{directory::DKGDirectory, session::DKGSession},
    manager::DKGManager,
};
use session::ccontext::CSessionCtx;
use std::sync::Arc;
use tokio::sync::Mutex;

// Networking.
type SOCKET = Arc<Mutex<tokio::net::TcpStream>>;
type PEER = Arc<Mutex<peer::Peer>>;
type PEER_MANAGER = Arc<Mutex<peer_manager::PeerManager>>;
type DKG_MANAGER = Arc<Mutex<DKGManager>>;
type DKG_DIRECTORY = Arc<Mutex<DKGDirectory>>;
type DKG_SESSION = Arc<Mutex<DKGSession>>;
type CSESSION_CTX = Arc<Mutex<CSessionCtx>>;
type BLAMING_DIRECTORY = Arc<Mutex<BlamingDirectory>>;

// Inscriptive

#[path = "inscriptive/baked.rs"]
pub mod baked;
#[path = "inscriptive/blaming.rs"]
pub mod blaming;
#[path = "inscriptive/registery/mod.rs"]
pub mod registery;

// Crypto modules.
#[path = "inscriptive/encoding/mod.rs"]
pub mod encoding;

#[path = "transmutive/hash.rs"]
pub mod hash;
#[path = "transmutive/into.rs"]
pub mod into;
#[path = "transmutive/key.rs"]
pub mod key;
#[path = "transmutive/musig/mod.rs"]
pub mod musig;
#[path = "transmutive/noist/mod.rs"]
pub mod noist;
#[path = "transmutive/schnorr.rs"]
pub mod schnorr;

// Operating modes.
#[path = "operative/mode/coordinator/coordinator.rs"]
pub mod coordinator;
// Operating modes.
#[path = "operative/mode/coordinator/dkgops.rs"]
pub mod dkgops;

#[path = "operative/session/mod.rs"]
pub mod session;

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

// Networking.
#[path = "communicative/rpc/bitcoin-rpc.rs"]
pub mod bitcoin_rpc;
#[path = "communicative/nns/mod.rs"]
pub mod nns;
#[path = "communicative/peer/peer.rs"]
pub mod peer;
#[path = "communicative/peer/manager.rs"]
pub mod peer_manager;
#[path = "communicative/tcp/mod.rs"]
pub mod tcp;

// Constructive
#[path = "constructive/entry/combinator/mod.rs"]
pub mod combinator;
#[path = "constructive/entry/entry.rs"]
pub mod entry;
#[path = "constructive/taproot.rs"]
pub mod taproot;
#[path = "constructive/txn/mod.rs"]
pub mod txn;
#[path = "constructive/txn.rs"]
pub mod txn_old;
#[path = "constructive/txo/mod.rs"]
pub mod txo;
#[path = "constructive/valtype/mod.rs"]
pub mod valtype;

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
