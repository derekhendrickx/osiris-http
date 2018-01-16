use std::collections::HashMap;

use peer::Peer;
use info_hash::InfoHash;

#[derive(Debug, Clone)]
pub struct Torrents {
    torrents: HashMap<InfoHash, HashMap<Vec<u8>, Peer>>,
}

impl Torrents {
    pub fn new() -> Torrents {
        Torrents {
            torrents: HashMap::new(),
        }
    }

    pub fn has_torrent(&self, torrent: &InfoHash) -> bool {
        self.torrents.contains_key(torrent)
    }

    pub fn get_torrent(&self, info_hash: &InfoHash) -> Option<&HashMap<Vec<u8>, Peer>> {
        self.torrents.get(info_hash)
    }

    pub fn show_torrents(&self) {
        println!("{:?}", self.torrents)
    }

    pub fn add_torrent(&mut self, torrent: InfoHash) {
        self.torrents.entry(torrent).or_insert_with(HashMap::new);
    }

    pub fn add_peer(&mut self, info_hash: &InfoHash, peer: Peer) {
        let torrent = self.torrents.get_mut(info_hash).unwrap();

        torrent.entry(peer.get_id().to_vec()).or_insert(peer);
    }
}
