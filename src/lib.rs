use std::sync::Arc;
use tokio::sync::Mutex;

pub mod baked;

type NostrClient = Arc<Mutex<nostr_sdk::Client>>;
type TCPStream = Arc<Mutex<tokio::net::TcpStream>>;

// Crypto modules.
#[path = "transmutive/hash.rs"]
pub mod hash;

#[path = "transmutive/key.rs"]
pub mod key;

#[path = "transmutive/schnorr.rs"]
pub mod schnorr;

// Operating modes.
#[path = "operative/client.rs"]
pub mod client;

#[path = "operative/operator.rs"]
pub mod operator;

#[path = "operative/coordinator.rs"]
pub mod coordinator;

// Networking.
#[path = "communicative/nns/server.rs"]
pub mod nns_server;

#[path = "communicative/nns/client.rs"]
pub mod nns_client;

#[path = "communicative/nns/relay.rs"]
pub mod nns_relay;

#[path = "communicative/tcp.rs"]
pub mod tcp;

#[path = "communicative/upnp.rs"]
pub mod upnp;
