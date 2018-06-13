use std::net::IpAddr;

use announce_event::AnnounceEvent;
use info_hash::InfoHash;

pub struct AnnounceRequestBuilder {
    info_hash: InfoHash,
    peer_id: Vec<u8>,
    port: u16,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    compact: bool,
    no_peer_id: Option<bool>,
    event: Option<AnnounceEvent>,
    ip: Option<IpAddr>,
    numwant: Option<u16>,
    key: Option<String>,
    tracker_id: Option<String>,
}

impl AnnounceRequestBuilder {
    pub fn new(
        info_hash: Vec<u8>,
        peer_id: Vec<u8>,
        port: u16,
        uploaded: u64,
        downloaded: u64,
        left: u64,
        compact: bool,
    ) -> Self {
        let info_hash = InfoHash::new(info_hash);

        AnnounceRequestBuilder {
            info_hash,
            peer_id,
            port,
            uploaded,
            downloaded,
            left,
            compact,
            no_peer_id: None,
            event: None,
            ip: None,
            numwant: None,
            key: None,
            tracker_id: None,
        }
    }

    pub fn no_peer_id(mut self, no_peer_id: bool) -> Self {
        self.no_peer_id = Some(no_peer_id);
        self
    }

    pub fn event(mut self, event: &str) -> Self {
        let event = match &*event {
            "started" => AnnounceEvent::Started,
            "stopped" => AnnounceEvent::Stopped,
            "completed" => AnnounceEvent::Completed,
            _ => AnnounceEvent::None,
        };

        self.event = Some(event);
        self
    }

    pub fn ip(mut self, ip: IpAddr) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn numwant(mut self, numwant: u16) -> Self {
        self.numwant = Some(numwant);
        self
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = Some(key);
        self
    }

    pub fn tracker_id(mut self, tracker_id: String) -> Self {
        self.tracker_id = Some(tracker_id);
        self
    }

    pub fn build(self) -> AnnounceRequest {
        AnnounceRequest {
            info_hash: self.info_hash,
            peer_id: self.peer_id,
            port: self.port,
            uploaded: self.uploaded,
            downloaded: self.downloaded,
            left: self.left,
            compact: self.compact,
            no_peer_id: self.no_peer_id,
            event: self.event,
            ip: self.ip,
            numwant: self.numwant,
            key: self.key,
            tracker_id: self.tracker_id,
        }
    }
}

#[derive(Debug)]
pub struct AnnounceRequest {
    info_hash: InfoHash,
    peer_id: Vec<u8>,
    port: u16,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    compact: bool,
    no_peer_id: Option<bool>,
    event: Option<AnnounceEvent>,
    ip: Option<IpAddr>,
    numwant: Option<u16>,
    key: Option<String>,
    tracker_id: Option<String>,
}

impl AnnounceRequest {
    pub fn get_info_hash(&self) -> &InfoHash {
        &self.info_hash
    }

    pub fn get_peer_id(&self) -> &Vec<u8> {
        &self.peer_id
    }

    pub fn get_ip(&self) -> &Option<IpAddr> {
        &self.ip
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_uploaded(&self) -> u64 {
        self.uploaded
    }

    pub fn get_downloaded(&self) -> u64 {
        self.downloaded
    }

    pub fn get_left(&self) -> u64 {
        self.left
    }

    pub fn get_compact(&self) -> bool {
        self.compact
    }

    pub fn get_no_peer_id(&self) -> Option<bool> {
        self.no_peer_id
    }

    pub fn get_event(&self) -> &Option<AnnounceEvent> {
        &self.event
    }

    pub fn get_numwant(&self) -> Option<u16> {
        self.numwant
    }

    pub fn get_key(&self) -> &Option<String> {
        &self.key
    }

    pub fn get_tracker_id(&self) -> &Option<String> {
        &self.tracker_id
    }
}
