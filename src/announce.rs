use std::net::{IpAddr, Ipv4Addr};

use hyper::{Body, Request};
use qstring::QString;

use announce_request::{AnnounceRequest, AnnounceRequestBuilder};
use announce_response::AnnounceResponse;
use connection_info::ConnectionInfo;
use helper::{get_param, get_param_as_bytes};
use peer::Peer;
use torrents::Torrents;

pub struct Announce;

impl Announce {
    pub fn announce(torrents: &mut Torrents, request: &Request<Body>) -> Body {
        let announce_request = Announce::parse_request(&request);
        let info_hash = announce_request.get_info_hash();
        let peer = Peer::new(&announce_request);

        // TODO: Announce events (https://www.blogsolute.com/what-is-torrent-tracker-how-it-works-detail/20187/):
        // 
        // started: The first request to the tracker must include the event key with this value.
        // stopped: Must be sent to the tracker if the client is shutting down gracefully.
        // completed: Must be sent to the tracker when the download completes. However, must not be sent if the download was already 100% complete when the client started. Presumably, this is to allow the tracker to increment the “completed downloads” metric based solely on this event.

        torrents.add_torrent(info_hash.clone());
        torrents.add_peer(info_hash, peer.clone());

        let peers = torrents.get_peers(info_hash, &peer);

        let announce_response = AnnounceResponse::new(
            &peers,
            announce_request.get_compact(),
            torrents.get_complete(info_hash),
            torrents.get_incomplete(info_hash),
        );

        let body = announce_response.bencode();

        // let body = (ben_map!{
        //     "failure reason" => ben_bytes!("Tracker offline")
        // }).encode();

        Body::from(body)
    }

    fn parse_request(request: &Request<Body>) -> AnnounceRequest {
        let mut query_string = QString::from("");
        let mut ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let connection_info = ConnectionInfo::get(&request);

        match connection_info.remote_addr() {
            Some(socket) => ip = socket.ip(),
            None => error!("No IP"),
        }

        match request.uri().query() {
            Some(str) => query_string = QString::from(str),
            None => error!("Query: None"),
        }

        let info_hash = get_param_as_bytes(&query_string, "info_hash").unwrap();

        let peer_id = match get_param_as_bytes(&query_string, "peer_id") {
            Some(bytes) => bytes,
            None => get_param(&query_string, "peer_id").as_bytes().to_vec(),
        };

        let port = get_param(&query_string, "port").parse().unwrap();

        let uploaded = get_param(&query_string, "uploaded").parse().unwrap();
        let downloaded = get_param(&query_string, "downloaded").parse().unwrap();
        let left = get_param(&query_string, "left").parse().unwrap();
        let compact = get_param(&query_string, "compact").parse::<u8>().unwrap() == 1;

        let no_peer_id = match get_param(&query_string, "no_peer_id").parse::<u8>() {
            Err(_error) => false,
            Ok(value) => value == 1,
        };

        let event = get_param(&query_string, "event");

        let ip_str = get_param(&query_string, "ip");
        if !ip_str.is_empty() {
            ip = get_param(&query_string, "ip").parse::<IpAddr>().unwrap();
        }

        let numwant = match get_param(&query_string, "numwant").parse() {
            Err(_error) => 0,
            Ok(value) => value,
        };

        let key = get_param(&query_string, "key");
        let tracker_id = get_param(&query_string, "trackerid");

        AnnounceRequestBuilder::new(
            &info_hash, &peer_id, port, uploaded, downloaded, left, compact,
        ).no_peer_id(no_peer_id)
            .event(event)
            .ip(&ip)
            .numwant(numwant)
            .key(key)
            .tracker_id(tracker_id)
            .build()
    }
}
