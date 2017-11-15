extern crate hyper;

use hyper::server::{Response};

pub struct Scrape;

impl Scrape {
	pub fn scrape(query: Option<&str>, response: &mut Response) {
		println!("Scrape");
		match query {
			Some(str) => println!("Query: {:}", str),
			None => println!("Query: None"),
		}
		response.set_body("Scrape the tracker");
	}
}
