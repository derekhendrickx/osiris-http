extern crate hyper;
extern crate qstring;

use hyper::server::{Response};
use self::qstring::QString;

pub struct Announce;

impl Announce {
	pub fn announce(query: Option<&str>, response: &mut Response) {
		println!("Announce");
		let mut query_string = QString::from("");

		match query {
			Some(str) => query_string = QString::from(str),
			None => println!("Query: None"),
		}

		println!("Query: {:}", query_string);

		response.set_body("Announce to the tracker\nd5:filesd20:xxxxxxxxxxxxxxxxxxxxd8:completei2e10:downloadedi0e10:incompletei4e4:name12:xxxxxxxxxxxxee5:flagsd20:min_request_intervali3600eee");
	}
}
