use std::collections::HashMap;

use peer::Peer;

#[derive(Debug, Clone)]
pub struct Torrents {
    torrents: HashMap<String, HashMap<String, Box<Peer>>>,
}

impl Torrents {
    pub fn new() -> Torrents {
        Torrents {
            torrents: HashMap::new(),
        }
    }

    pub fn has_torrent(&self, torrent: &str) -> bool {
        self.torrents.contains_key(torrent)
    }

    pub fn get_torrent(&self, info_hash: &str) -> Option<&HashMap<String, Box<Peer>>> {
        self.torrents.get(info_hash)
    }

    pub fn add_torrent(&mut self, torrent: &str) {
        self.torrents.insert(torrent.to_string(), HashMap::new());
    }

    pub fn add_peer(&mut self, info_hash: &str, peer: Box<Peer>) {
        let peers = self.torrents.get_mut(info_hash).unwrap();
        peers.entry(peer.get_id().to_string()).or_insert(peer);
    }
}
