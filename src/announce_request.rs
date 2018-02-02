use std::net::IpAddr;
use url::percent_encoding::percent_decode;

use qstring::QString;

use info_hash::InfoHash;
use announce_event::AnnounceEvent;

#[derive(Debug)]
pub struct AnnounceRequest {
    info_hash: InfoHash,
    peer_id: Vec<u8>,
    port: u16,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    compact: bool,
    no_peer_id: bool,
    event: AnnounceEvent,
    ip: IpAddr,
    numwant: u16,
    key: String,
    trackerid: String,
}

impl AnnounceRequest {
    pub fn new(data: &QString, ip: &IpAddr) -> AnnounceRequest {
        let ip_str = get_param(data, "ip");
        let mut announce_request_ip = *ip;

        let info_hash = InfoHash::new(get_param_as_bytes(data, "info_hash").unwrap());

        let peer_id = match get_param_as_bytes(data, "peer_id") {
            Some(bytes) => bytes,
            None => get_param(data, "peer_id").as_bytes().to_vec(),
        };

        let event = match &*get_param(data, "event") {
            "started" => AnnounceEvent::Started,
            "stopped" => AnnounceEvent::Stopped,
            "completed" => AnnounceEvent::Completed,
            _ => AnnounceEvent::None,
        };

        if !ip_str.is_empty() {
            announce_request_ip = get_param(data, "ip").parse::<IpAddr>().unwrap();
        }

        let no_peer_id = match get_param(data, "no_peer_id").parse::<u8>() {
            Err(_error) => false,
            Ok(value) => value == 1,
        };

        let numwant = match get_param(data, "numwant").parse() {
            Err(_error) => 0,
            Ok(value) => value,
        };

        println!(
            "Info hash = {:?}\tLength = {}",
            info_hash.get_hash(),
            info_hash.get_hash().len()
        );
        println!("Peer id = {:?}\tLength = {}", peer_id, peer_id.len());

        AnnounceRequest {
            info_hash,
            peer_id,
            port: get_param(data, "port").parse().unwrap(),
            uploaded: get_param(data, "uploaded").parse().unwrap(),
            downloaded: get_param(data, "downloaded").parse().unwrap(),
            left: get_param(data, "left").parse().unwrap(),
            compact: get_param(data, "compact").parse::<u8>().unwrap() == 1,
            no_peer_id,
            event,
            ip: announce_request_ip,
            numwant,
            key: get_param(data, "key").to_string(),
            trackerid: String::from(""),
        }
    }

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

    pub fn get_no_peer_id(&self) -> bool {
        self.no_peer_id
    }

    pub fn get_event(&self) -> &AnnounceEvent {
        &self.event
    }

    pub fn get_numwant(&self) -> u16 {
        self.numwant
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_trackerid(&self) -> &str {
        &self.trackerid
    }
}

fn get_param<'a>(data: &'a QString, param: &'a str) -> &'a str {
    match data.get(param) {
        None => "",
        Some(ip) => ip,
    }
}

fn get_param_as_bytes(data: &QString, param: &str) -> Option<Vec<u8>> {
    let param_as_str = get_param(data, param).as_bytes();

    percent_decode(param_as_str).if_any()
}
