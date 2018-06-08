use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr};

use hyper::{Request, Response, Body, HeaderMap};
use hyper::header::{CACHE_CONTROL, CONTENT_TYPE};
use byteorder::{BigEndian, WriteBytesExt};
use bip_bencode::{BMutAccess, BencodeMut};
use qstring::QString;

use announce_request::AnnounceRequest;
use torrents::Torrents;
use peer::Peer;

pub struct Announce;

fn bencode_response(peers: &[&Peer], compact: bool, complete: u32, incomplete: u32) -> Vec<u8> {
    let bencoded_peers = if compact {
        let mut peer_binary = vec![];
        peers.into_iter().for_each(|peer| {
            let ip = peer.get_ip();

            if ip.is_ipv4() {
                let ipv4 = Ipv4Addr::from_str(&ip.to_string()).unwrap();
                for number in &ipv4.octets() {
                    peer_binary.write_u8(*number).unwrap();
                }
            }

            peer_binary.write_u16::<BigEndian>(peer.get_port()).unwrap();
        });

        ben_bytes!(peer_binary)
    } else {
        let mut bencode_list = BencodeMut::new_list();
        {
            let list = bencode_list.list_mut().unwrap();
            peers.into_iter().for_each(|peer| {
                list.push(ben_map!{
                    "peer id" => ben_bytes!(peer.get_id().as_slice()),
                    "ip" => ben_bytes!(peer.get_ip().to_string()),
                    "port" => ben_int!(i64::from(peer.get_port()))
                });
            });
        }
        bencode_list
    };

    let message = ben_map!{
        "interval" => ben_int!(30),
        "complete" => ben_int!(i64::from(complete)),
        "incomplete" => ben_int!(i64::from(incomplete)),
        "peers" => bencoded_peers
    };

    message.encode()
}

impl Announce {
    pub fn announce(torrents: &mut Torrents, request: &Request<Body>) -> Body {
        let mut query_string = QString::from("");
        let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

        // match request.remote_addr() {
        //     Some(socket) => ip = socket.ip(),
        //     None => error!("No IP"),
        // }

        match request.uri().query() {
            Some(str) => query_string = QString::from(str),
            None => error!("Query: None"),
        }

        let announce_request = AnnounceRequest::new(&query_string, &ip);
        let info_hash = announce_request.get_info_hash();
        let peer = Peer::new(&announce_request);

        torrents.add_torrent(info_hash.clone());
        torrents.add_peer(info_hash, peer.clone());

        let peers = torrents.get_peers(info_hash, &peer);

        let body = bencode_response(
            &peers,
            announce_request.get_compact(),
            torrents.get_complete(info_hash),
            torrents.get_incomplete(info_hash),
        );

        // let body = (ben_map!{
        //     "failure reason" => ben_bytes!("Tracker offline")
        // }).encode();

        let mut headers = HeaderMap::new();
        headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
        headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());

        Body::from(body)
    }
}
