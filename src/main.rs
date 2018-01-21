#[macro_use]
extern crate bip_bencode;
extern crate env_logger;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate url;

use std::sync::{Arc, Mutex};
use std::net::Ipv4Addr;
use hyper::server::Http;

use torrents::Torrents;

mod router;
mod torrents;
mod announce;
mod scrape;
mod peer;
mod info_hash;

fn main() {
    env_logger::init().unwrap();
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let addr = (ip.to_string() + ":6969").parse().unwrap();
    let torrents = Arc::new(Mutex::new(Torrents::new()));
    let server = Http::new()
        .bind(&addr, move || {
            let routes = router::Router::new(Arc::clone(&torrents));
            Ok(routes)
        })
        .unwrap();
    info!("Tracker running on {}...", addr);
    server.run().unwrap();
}
