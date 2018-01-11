use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Peer {
    id: String,
    port: u16,
    ip: IpAddr,
    uploaded: u64,
    downloaded: u64,
    left: u64,
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

    pub fn get_id(&self) -> &str {
        &self.id
    }
}
