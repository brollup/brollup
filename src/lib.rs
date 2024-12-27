pub mod baked;

// Crypto modules.
#[path = "transmutive/hash.rs"]
pub mod hash;

#[path = "transmutive/key.rs"]
pub mod key;

#[path = "transmutive/schnorr.rs"]
pub mod schnorr;

#[path = "transmutive/noist/vse.rs"]
pub mod vse;

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
