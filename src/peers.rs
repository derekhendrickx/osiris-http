use std::net::IpAddr;
use std::collections::HashMap;

pub struct Peer {
    pub id: String,
    port: u16,
    ip: IpAddr,
    uploaded: u32,
    downloaded: u32,
    left: u32,
}

impl Peer {
    pub fn new(id: &str, port: u16, ip: IpAddr) -> Peer {
        Peer {
            id: String::from(id),
            port,
            ip,
            uploaded: 0,
            downloaded: 0,
            left: 0,
        }
    }
}

struct File {
    info_hash: String,
    peers: HashMap<String, Peer>,
}

impl File {
    fn new(info_hash: String) -> File {
        File {
            info_hash,
            peers: HashMap::new(),
        }
    }
}
