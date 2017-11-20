extern crate hyper;
extern crate qstring;

use std::net::{IpAddr, Ipv4Addr};
use hyper::server::{Request, Response};
use hyper::header::{Headers, ContentLength, ContentType, CacheControl, CacheDirective};
use hyper::mime;
use self::qstring::QString;

pub struct Announce;

enum AnnounceEvent {
    Started,
	Stopped,
	Completed,
}

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
	trackerid: String
}

impl AnnounceRequest {
	fn new(data: &QString) -> Self {
		AnnounceRequest {
			info_hash: (&data["info_hash"]).to_string(),
			peer_id: (&data["peer_id"]).to_string(),
			port: (&data["port"]).parse().unwrap(),
			uploaded: (&data["uploaded"]).parse().unwrap(),
			downloaded: (&data["downloaded"]).parse().unwrap(),
			left: (&data["left"]).parse().unwrap(),
			compact: (&data["compact"]).parse::<u8>().unwrap() == 1,
			no_peer_id: (&data["no_peer_id"]).parse::<u8>().unwrap() == 1,
			event: AnnounceEvent::Started,
			ip: (&data["ip"]).parse::<IpAddr>().unwrap(),
			numwant: (&data["numwant"]).parse().unwrap(),
			key: (&data["key"]).to_string(),
			trackerid: (&data["trackerid"]).to_string()
		}
	}
}

impl Announce {
	pub fn announce(request: Request) -> Response {
		println!("Announce");
		let mut query_string = QString::from("");
		let mut ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

		match request.remote_addr() {
			Some(socket) => ip = socket.ip(),
			None => println!("Query: None"),
		}

		match request.query() {
			Some(str) => query_string = QString::from(str),
			None => println!("Query: None"),
		}

		let announce_request = AnnounceRequest::new(&query_string);

		println!("Query:");
		println!("\tinfo_hash = {}", announce_request.info_hash);
		println!("\tpeer_id = {}", announce_request.peer_id);
		println!("\tport = {}", announce_request.port);
		println!("\tuploaded = {}", announce_request.uploaded);
		println!("\tdownloaded = {}", announce_request.downloaded);
		println!("\tleft = {}", announce_request.left);
		println!("\tcompact = {}", announce_request.compact);
		println!("\tno_peer_id = {}", announce_request.no_peer_id);
		// println!("\tevent = {}", announce_request.event);
		println!("\tip = {}", announce_request.ip);
		println!("\tnumwant = {}", announce_request.numwant);
		println!("\tkey = {}", announce_request.key);
		println!("\ttrackerid = {}", announce_request.trackerid);

		let peers = ben_map!{
			"peer_id" => ben_bytes!(announce_request.peer_id),
			"ip" => ben_bytes!(ip.to_string()),
			"port" => ben_int!(announce_request.port as i64)
		};
		let message = (ben_map!{
            "interval" => ben_int!(30),
			"complete" => ben_int!(1),
			"incomplete" => ben_int!(0),
			"peers" => peers
        }).encode();
		let message = "Test";

		// let message = (ben_map!{
		// 	"failure reason" => ben_bytes!("Tracker offline")
		// }).encode();

		// response.set_body("Announce to the tracker\nd5:filesd20:xxxxxxxxxxxxxxxxxxxxd8:completei2e10:downloadedi0e10:incompletei4e4:name12:xxxxxxxxxxxxee5:flagsd20:min_request_intervali3600eee");
		let mut headers = Headers::new();
		headers.set(ContentLength(message.len() as u64));
		headers.set(CacheControl(vec![CacheDirective::NoCache]));
		headers.set(ContentType(mime::TEXT_PLAIN));

		Response::new()
			.with_headers(headers)
			.with_body(message)
	}
}
