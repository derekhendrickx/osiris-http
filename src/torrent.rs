use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Torrent {
    info_hash: String,
}

impl Torrent {
    pub fn new(info_hash: &str) -> Torrent {
        Torrent {
            info_hash: String::from(info_hash),
        }
    }

    pub fn get_info_hash(&self) -> &str {
        &self.info_hash
    }
}
