use std::net::IpAddr;

#[derive(Debug)]
#[derive(Clone)]
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
