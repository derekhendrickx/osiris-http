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

use tracker::Tracker;
use peers::Peer;

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
    info_hash: String,
    peer_id: String,
    port: u16,
    uploaded: u32,
    downloaded: u32,
    left: u32,
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
        let ip_str = &data["ip"];
        let mut announce_request_ip = *ip;

        let event = match &*data["event"] {
            "started" => AnnounceEvent::Started,
            "stopped" => AnnounceEvent::Stopped,
            "completed" => AnnounceEvent::Completed,
            _ => AnnounceEvent::None,
        };

        if !ip_str.is_empty() {
            announce_request_ip = (&data["ip"]).parse::<IpAddr>().unwrap();
        }

        AnnounceRequest {
            info_hash: (&data["info_hash"]).to_string(),
            peer_id: (&data["peer_id"]).to_string(),
            port: (&data["port"]).parse().unwrap(),
            uploaded: (&data["uploaded"]).parse().unwrap(),
            downloaded: (&data["downloaded"]).parse().unwrap(),
            left: (&data["left"]).parse().unwrap(),
            compact: (&data["compact"]).parse::<u8>().unwrap() == 1,
            no_peer_id: (&data["no_peer_id"]).parse::<u8>().unwrap() == 1,
            event,
            ip: announce_request_ip,
            numwant: (&data["numwant"]).parse().unwrap(),
            key: (&data["key"]).to_string(),
            trackerid: (&data["trackerid"]).to_string(),
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

impl Announce {
    pub fn announce(tracker: &mut Tracker, request: &Request) -> Response {
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

        let announce_request = AnnounceRequest::new(&query_string, &ip);

        info!("Announce\nRequest: {:?}", announce_request);

        let peer = Box::new(Peer::new(
            &announce_request.peer_id,
            announce_request.port,
            announce_request.ip,
        ));
        println!("Tracker before: {:?}", tracker);
        tracker.add_file(&announce_request.info_hash);
        tracker.add_peer(&announce_request.info_hash, peer);
        println!("Tracker after: {:?}", tracker);

        let body = announce_request.bencode();

        // let message = (ben_map!{
        // 	"failure reason" => ben_bytes!("Tracker offline")
        // }).encode();

        let mut headers = Headers::new();
        headers.set(ContentLength(body.len() as u64));
        headers.set(CacheControl(vec![CacheDirective::NoCache]));
        headers.set(ContentType(mime::TEXT_PLAIN));

        Response::new().with_headers(headers).with_body(body)
    }
}
