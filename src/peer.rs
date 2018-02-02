use std::net::IpAddr;

use announce_request::AnnounceRequest;

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
    pub fn new(announce_request: &AnnounceRequest) -> Peer {
        Peer {
            id: announce_request.get_peer_id().clone(),
            port: announce_request.get_port(),
            ip: *announce_request.get_ip(),
            uploaded: announce_request.get_uploaded(),
            downloaded: announce_request.get_downloaded(),
            left: announce_request.get_left(),
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

impl PartialEq for Peer {
    fn eq(&self, other: &Peer) -> bool {
        if self.id.len() != other.get_id().len() {
            return false;
        }

        self.id
            .iter()
            .enumerate()
            .any(|(index, &x)| x != other.get_id()[index])
    }
}

impl Eq for Peer {}
