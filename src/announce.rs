extern crate hyper;

use hyper::server::{Response};

pub struct Announce;

impl Announce {
	pub fn announce(query: Option<&str>, response: &mut Response) {
		println!("Announce");
		match query {
			Some(str) => println!("Query: {:}", str),
			None => println!("Query: None"),
		}
		response.set_body("Announce to the tracker\nd5:filesd20:xxxxxxxxxxxxxxxxxxxxd8:completei2e10:downloadedi0e10:incompletei4e4:name12:xxxxxxxxxxxxee5:flagsd20:min_request_intervali3600eee");
	}
}
