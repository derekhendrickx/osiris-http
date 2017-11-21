#[macro_use]
extern crate bip_bencode;
extern crate hyper;
extern crate local_ip;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::net::Ipv4Addr;
use hyper::server::Http;

mod tracker;
mod announce;
mod scrape;

fn main() {
    env_logger::init().unwrap();
    // let ip = local_ip::get().unwrap();
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let addr = (ip.to_string() + ":6969").parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(tracker::Tracker)).unwrap();
    info!("Tracker running on {}...", addr);
    server.run().unwrap();
}
