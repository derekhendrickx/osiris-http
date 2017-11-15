extern crate hyper;
extern crate qstring;

use hyper::server::{Response};
use self::qstring::QString;

pub struct Scrape;

impl Scrape {
	pub fn scrape(query: Option<&str>, response: &mut Response) {
		println!("Scrape");
		let mut query_string = QString::from("");

		match query {
			Some(str) => query_string = QString::from(str),
			None => println!("Query: None"),
		}

		println!("Query: {:}", query_string);

		response.set_body("Scrape the tracker");
	}
}