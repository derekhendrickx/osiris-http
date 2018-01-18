use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Peer {
    id: Vec<u8>,
    port: u16,
    ip: IpAddr,
    uploaded: u64,
    downloaded: u64,
    left: u64,
}

impl Peer {
    pub fn new(
        id: Vec<u8>,
        port: u16,
        ip: IpAddr,
        uploaded: u64,
        downloaded: u64,
        left: u64,
    ) -> Peer {
        Peer {
            id,
            port,
            ip,
            uploaded,
            downloaded,
            left,
        }
    }

    pub fn get_id(&self) -> &Vec<u8> {
        &self.id
    }

    pub fn get_ip(&self) -> &IpAddr {
        &self.ip
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_left(&self) -> u64 {
        self.left
    }
}
