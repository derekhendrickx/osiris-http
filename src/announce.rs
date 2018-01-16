extern crate byteorder;
extern crate hyper;
extern crate qstring;

use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr};
use hyper::server::{Request, Response};
use hyper::header::{CacheControl, CacheDirective, ContentLength, ContentType, Headers};
use hyper::mime;
use self::byteorder::{BigEndian, WriteBytesExt};
use self::qstring::QString;
use url::percent_encoding::percent_decode;

use torrents::Torrents;
use peer::Peer;
use info_hash::InfoHash;

pub struct Announce;

#[derive(Debug)]
enum AnnounceEvent {
    Started,
    Stopped,
    Completed,
    None,
}

#[derive(Debug)]
struct AnnounceRequest {
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
    fn new(data: &QString, ip: &IpAddr) -> AnnounceRequest {
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

    fn bencode(self) -> Vec<u8> {
        let peers = if self.compact {
            let mut peer_binary = vec![];

            if self.ip.is_ipv4() {
                let ipv4 = Ipv4Addr::from_str(&self.ip.to_string()).unwrap();
                for number in &ipv4.octets() {
                    peer_binary.write_u8(*number).unwrap();
                }
            }

            peer_binary.write_u16::<BigEndian>(self.port).unwrap();
            ben_bytes!(peer_binary)
        } else {
            let peers_dictionnary = ben_map!{
                "peer id" => ben_bytes!(self.peer_id),
                "ip" => ben_bytes!(self.ip.to_string()),
                "port" => ben_int!(i64::from(self.port))
            };
            ben_list!(peers_dictionnary)
        };

        let message = ben_map!{
            "interval" => ben_int!(30),
            "complete" => ben_int!(1),
            "incomplete" => ben_int!(0),
            "peers" => peers
        };

        message.encode()
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

impl Announce {
    pub fn announce(torrents: &mut Torrents, request: &Request) -> Response {
        let mut query_string = QString::from("");
        let mut ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

        match request.remote_addr() {
            Some(socket) => ip = socket.ip(),
            None => error!("No IP"),
        }

        match request.query() {
            Some(str) => query_string = QString::from(str),
            None => error!("Query: None"),
        }
        println!("{:?}", query_string);

        let announce_request = AnnounceRequest::new(&query_string, &ip);

        let peer = Peer::new(
            announce_request.peer_id,
            announce_request.port,
            announce_request.ip,
            announce_request.uploaded,
            announce_request.downloaded,
            announce_request.left,
        );
        println!("Tracker before:");
        torrents.show_torrents();
        torrents.add_torrent(announce_request.info_hash.clone());
        torrents.add_peer(&announce_request.info_hash, peer);
        println!("Tracker after:");
        torrents.show_torrents();

        // let body = announce_request.bencode();

        let body = (ben_map!{
            "failure reason" => ben_bytes!("Tracker offline")
        }).encode();

        let mut headers = Headers::new();
        headers.set(ContentLength(body.len() as u64));
        headers.set(CacheControl(vec![CacheDirective::NoCache]));
        headers.set(ContentType(mime::TEXT_PLAIN));

        Response::new().with_headers(headers).with_body(body)
    }
}
