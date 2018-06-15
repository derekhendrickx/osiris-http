use std::net::{IpAddr, Ipv4Addr};

use announce_event::AnnounceEvent;
use info_hash::InfoHash;
use torrents::Torrents;

pub struct AnnounceRequestBuilder<'a> {
    info_hash: &'a [u8],
    peer_id: &'a [u8],
    port: u16,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    compact: bool,
    no_peer_id: Option<bool>,
    event: Option<&'a str>,
    ip: Option<&'a IpAddr>,
    numwant: Option<u16>,
    key: Option<&'a str>,
    tracker_id: Option<&'a str>,
}

impl<'a> AnnounceRequestBuilder<'a> {
    pub fn new(
        info_hash: &'a [u8],
        peer_id: &'a [u8],
        port: u16,
        uploaded: u64,
        downloaded: u64,
        left: u64,
        compact: bool,
    ) -> Self {
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

    pub fn event(mut self, event: &'a str) -> Self {
        self.event = Some(event);
        self
    }

    pub fn ip(mut self, ip: &'a IpAddr) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn numwant(mut self, numwant: u16) -> Self {
        self.numwant = Some(numwant);
        self
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn tracker_id(mut self, tracker_id: &'a str) -> Self {
        self.tracker_id = Some(tracker_id);
        self
    }

    pub fn build(self) -> AnnounceRequest {
        let event = match &*self.event.unwrap_or("") {
            "started" => AnnounceEvent::Started,
            "stopped" => AnnounceEvent::Stopped,
            "completed" => AnnounceEvent::Completed,
            _ => AnnounceEvent::None,
        };
        let loopback = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let ip = self.ip.unwrap_or(&loopback);

        AnnounceRequest {
            info_hash: InfoHash::new(self.info_hash),
            peer_id: self.peer_id.to_owned(),
            port: self.port,
            uploaded: self.uploaded,
            downloaded: self.downloaded,
            left: self.left,
            compact: self.compact,
            no_peer_id: self.no_peer_id,
            event,
            ip: ip.to_owned(),
            numwant: self.numwant,
            key: self.key.map(str::to_string),
            tracker_id: self.tracker_id.map(str::to_string),
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
    event: AnnounceEvent,
    ip: IpAddr,
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

    pub fn get_ip(&self) -> &IpAddr {
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

    pub fn get_event(&self) -> &AnnounceEvent {
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

    pub fn handle_event(&self, torrents: &mut Torrents) {
        self.event.handle(self, torrents);
    }
}
