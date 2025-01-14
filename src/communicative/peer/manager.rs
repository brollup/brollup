use crate::{
    nns::client::NNSClient,
    peer::{Peer, PeerKind},
    PEER, SOCKET,
};
use futures::{future::join_all, lock::Mutex};
use std::{collections::HashMap, sync::Arc};

pub struct PeerManager {
    peers: HashMap<[u8; 32], PEER>,
    nns_client: NNSClient,
}

impl PeerManager {
    pub async fn new(
        nns_client: &NNSClient,
        kind: PeerKind,
        keys: &Vec<[u8; 32]>,
        min: u64,
    ) -> Option<Self> {
        let mut manager = PeerManager {
            peers: HashMap::<[u8; 32], PEER>::new(),
            nns_client: nns_client.to_owned(),
        };
        if manager.add_peers(kind, keys).await < min {
            return None;
        }
        Some(manager)
    }

    async fn insert_peer(&mut self, peer: PEER) -> bool {
        let peer_key = {
            let _peer = peer.lock().await;
            _peer.key()
        };

        if self.peers.contains_key(&peer_key) {
            return false;
        }

        self.peers.insert(peer_key, Arc::clone(&peer));

        true
    }

    pub fn peers(&self) -> HashMap<[u8; 32], PEER> {
        self.peers.clone()
    }

    pub fn retrieve_peer(&self, key: [u8; 32]) -> Option<PEER> {
        Some(Arc::clone(self.peers.get(&key)?))
    }

    pub fn retrieve_peers(&self, keys: &Vec<[u8; 32]>) -> Option<Vec<PEER>> {
        let mut peers = Vec::<PEER>::new();

        for key in keys {
            if let Some(peer) = self.retrieve_peer(key.to_owned()) {
                peers.push(peer);
            }
        }
        Some(peers)
    }

    pub fn is_peer(&self, key: [u8; 32]) -> bool {
        match self.retrieve_peer(key) {
            Some(_) => return true,
            None => return false,
        }
    }

    pub async fn peer_socket(&self, key: [u8; 32]) -> Option<SOCKET> {
        let peer = self.retrieve_peer(key)?;
        let _peer = peer.lock().await;
        _peer.socket()
    }

    pub async fn is_peer_connected(&self, key: [u8; 32]) -> bool {
        let peer = match self.retrieve_peer(key) {
            Some(peer) => peer,
            None => return false,
        };

        let conn = {
            let _peer = peer.lock().await;
            _peer.connection()
        };

        match conn {
            Some(_) => return true,
            None => return false,
        }
    }

    /// Tries to connect to a list of peers and returns the number of peers connected.
    pub async fn add_peers(&mut self, kind: PeerKind, keys: &Vec<[u8; 32]>) -> u64 {
        let peer_list_ = Arc::new(Mutex::new(Vec::<PEER>::new()));

        let mut tasks = vec![];

        for key in keys.iter() {
            if self.is_peer(key.to_owned()) {
                continue;
            }

            let peer_list_ = Arc::clone(&peer_list_);
            let kind = kind.clone();
            let key = key.clone();
            let nns_client = self.nns_client.clone();

            tasks.push(tokio::spawn(async move {
                let peer: PEER = match Peer::connect(kind, key, &nns_client).await {
                    Ok(peer) => peer,
                    Err(_) => return,
                };

                {
                    let mut _peer_list_ = peer_list_.lock().await;
                    _peer_list_.push(peer);
                }
            }));
        }

        join_all(tasks).await;

        let peer_list: Vec<PEER> = {
            let _peer_list = peer_list_.lock().await;
            (*_peer_list).clone()
        };

        let peer_list_len = peer_list.len() as u64;

        for peer in peer_list {
            self.insert_peer(peer).await;
        }

        peer_list_len
    }
}
