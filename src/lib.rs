#![allow(non_camel_case_types)]

use blacklist::BlacklistDirectory;
use epoch::dir::EpochDirectory;
use lp::dir::LPDirectory;
use noist::{
    dkg::{directory::DKGDirectory, session::DKGSession},
    manager::DKGManager,
};
use session::ccontext::CSessionCtx;
use std::sync::Arc;
use tokio::sync::Mutex;
use wallet::{lift::LiftWallet, vtxo::VTXOWallet};

// Networking.
type SOCKET = Arc<Mutex<tokio::net::TcpStream>>;
type PEER = Arc<Mutex<peer::Peer>>;
type PEER_MANAGER = Arc<Mutex<peer_manager::PeerManager>>;
type DKG_MANAGER = Arc<Mutex<DKGManager>>;
type DKG_DIRECTORY = Arc<Mutex<DKGDirectory>>;
type DKG_SESSION = Arc<Mutex<DKGSession>>;
type CSESSION_CTX = Arc<Mutex<CSessionCtx>>;

// Wallets
type LIFT_WALLET = Arc<Mutex<LiftWallet>>;
type VTXO_WALLET = Arc<Mutex<VTXOWallet>>;

// Dirs
type LP_DIRECTORY = Arc<Mutex<LPDirectory>>;
type BLIST_DIRECTORY = Arc<Mutex<BlacklistDirectory>>;
type EPOCH_DIRECTORY = Arc<Mutex<EpochDirectory>>;

// Inscriptive

#[path = "inscriptive/baked.rs"]
pub mod baked;
#[path = "inscriptive/blacklist.rs"]
pub mod blacklist;
#[path = "inscriptive/epoch/mod.rs"]
pub mod epoch;
#[path = "inscriptive/lp/mod.rs"]
pub mod lp;
#[path = "inscriptive/registery/mod.rs"]
pub mod registery;
#[path = "inscriptive/wallet/mod.rs"]
pub mod wallet;

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
#[path = "operative/mode/node/node.rs"]
pub mod node;
#[path = "operative/mode/operator/operator.rs"]
pub mod operator;
#[path = "operative/mode/node/scanner.rs"]
pub mod scanner;
#[path = "operative/session/mod.rs"]
pub mod session;

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
#[path = "communicative/peer/peer.rs"]
pub mod peer;
#[path = "communicative/peer/manager.rs"]
pub mod peer_manager;
#[path = "communicative/rpc/mod.rs"]
pub mod rpc;
#[path = "communicative/rpc/bitcoin/rpcholder.rs"]
pub mod rpcholder;
#[path = "communicative/tcp/mod.rs"]
pub mod tcp;

// Constructive
#[path = "constructive/entry/combinator/mod.rs"]
pub mod combinator;
#[path = "constructive/entry/entry.rs"]
pub mod entry;
#[path = "constructive/txn/prevout.rs"]
pub mod prevout;
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

impl ToString for Network {
    fn to_string(&self) -> String {
        match self {
            Network::Signet => "signet".to_string(),
            Network::Mainnet => "mainnet".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperatingMode {
    Node,
    Operator,
    Coordinator,
}
