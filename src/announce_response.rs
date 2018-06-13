use std::net::Ipv4Addr;
use std::str::FromStr;

use bip_bencode::{BMutAccess, BencodeMut};
use byteorder::{BigEndian, WriteBytesExt};

use peer::Peer;

pub struct AnnounceResponse<'a> {
    peers: &'a [&'a Peer],
    compact: bool,
    complete: u32,
    incomplete: u32,
}

impl<'a> AnnounceResponse<'a> {
    pub fn new(peers: &'a [&'a Peer], compact: bool, complete: u32, incomplete: u32) -> Self {
        AnnounceResponse {
            peers,
            compact,
            complete,
            incomplete,
        }
    }

    pub fn bencode(&self) -> Vec<u8> {
        let bencoded_peers = if self.compact {
            let mut peer_binary = vec![];
            self.peers.into_iter().for_each(|peer| {
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
                self.peers.into_iter().for_each(|peer| {
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
            "complete" => ben_int!(i64::from(self.complete)),
            "incomplete" => ben_int!(i64::from(self.incomplete)),
            "peers" => bencoded_peers
        };

        message.encode()
    }
}
