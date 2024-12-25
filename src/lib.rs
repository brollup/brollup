pub mod baked;

// Crypto modules.
#[path = "transmutive/hash.rs"]
pub mod hash;

#[path = "transmutive/key.rs"]
pub mod key;

#[path = "transmutive/schnorr.rs"]
pub mod schnorr;

// Operating modes.
#[path = "operative/node.rs"]
pub mod node;

#[path = "operative/operator.rs"]
pub mod operator;

#[path = "operative/coordinator.rs"]
pub mod coordinator;

// Networking.
#[path = "communicative/nns/server.rs"]
pub mod nns_server;

#[path = "communicative/nns/query.rs"]
pub mod nns_query;

#[path = "communicative/nns/relay.rs"]
pub mod nns_relay;

#[path = "communicative/tcp.rs"]
pub mod tcp;

#[path = "communicative/request.rs"]
pub mod tcp_request;

#[path = "communicative/server.rs"]
pub mod tcp_server;

#[path = "communicative/peer.rs"]
pub mod peer;

#[derive(Copy, Clone)]
pub enum OperatingMode {
    Coordinator,
    Operator,
    Node,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PeerKind {
    Coordinator,
    Operator,
    Indexer,
    Node,
}
