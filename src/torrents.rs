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

    pub fn has_file(&self, file: &str) -> bool {
        self.torrents.contains_key(file)
    }

    pub fn add_file(&mut self, file: &str) {
        self.torrents.insert(file.to_string(), HashMap::new());
    }

    pub fn add_peer(&mut self, info_hash: &str, peer: Box<Peer>) {
        let peers = self.torrents.get_mut(info_hash).unwrap();
        peers.entry(peer.get_id().to_string()).or_insert(peer);
    }
}
