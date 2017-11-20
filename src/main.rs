#[macro_use]
extern crate bip_bencode;
extern crate hyper;

use hyper::server::{Http};

mod tracker;
mod announce;
mod scrape;

fn main() {
    let addr = "127.0.0.1:6969".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(tracker::Tracker)).unwrap();
    server.run().unwrap();
}
