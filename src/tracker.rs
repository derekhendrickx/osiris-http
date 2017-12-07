use std::collections::HashMap;

use peers::Peer;

#[derive(Debug)]
#[derive(Clone)]
pub struct Tracker {
    files: HashMap<String, HashMap<String, Box<Peer>>>,
}

impl Tracker {
    pub fn new() -> Tracker {
        Tracker { files: HashMap::new() }
    }

    pub fn has_file(&self, file: &str) -> bool {
        self.files.contains_key(file)
    }

    pub fn add_file(&mut self, file: &str) {
        self.files.insert(file.to_string(), HashMap::new());
    }

    pub fn add_peer(&mut self, info_hash: &str, peer: Box<Peer>) {
        let peers = self.files.get_mut(info_hash).unwrap();
        peers.entry(peer.id.to_string()).or_insert(peer);
    }
}
