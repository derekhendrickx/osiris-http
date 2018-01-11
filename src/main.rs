#[macro_use]
extern crate bip_bencode;
extern crate env_logger;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};
use std::net::Ipv4Addr;
use hyper::server::Http;

use tracker::Tracker;

mod router;
mod tracker;
mod announce;
mod scrape;
mod peers;

fn main() {
    env_logger::init().unwrap();
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let addr = (ip.to_string() + ":6969").parse().unwrap();
    let tracker = Arc::new(Mutex::new(Tracker::new()));
    let server = Http::new()
        .bind(&addr, move || {
            let routes = router::Router::new(Arc::clone(&tracker));
            Ok(routes)
        })
        .unwrap();
    info!("Tracker running on {}...", addr);
    server.run().unwrap();
}
