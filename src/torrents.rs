use std::collections::HashMap;

use info_hash::InfoHash;
use peer::Peer;

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

    pub fn get_peers(&self, info_hash: &InfoHash, current_peer: &Peer) -> Vec<&Peer> {
        let torrent = self.get_torrent(info_hash).unwrap();

        torrent
            .values()
            .filter_map(|peer| {
                if peer == current_peer && peer.get_left() == 0 {
                    Some(peer)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_complete(&self, info_hash: &InfoHash) -> u32 {
        let torrent = self.get_torrent(info_hash).unwrap();

        torrent.values().fold(0, |sum, peer| {
            let complete = if peer.get_left() == 0 { 1 } else { 0 };
            sum + complete
        })
    }

    pub fn get_incomplete(&self, info_hash: &InfoHash) -> u32 {
        let torrent = self.get_torrent(info_hash).unwrap();

        torrent.values().fold(0, |sum, peer| {
            let incomplete = if peer.get_left() > 0 { 1 } else { 0 };
            sum + incomplete
        })
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
