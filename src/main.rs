#[macro_use]
extern crate bip_bencode;
extern crate byteorder;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate qstring;
extern crate url;

use std::sync::{Arc, Mutex};
use std::net::Ipv4Addr;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::Server;

use torrents::Torrents;

mod announce;
mod announce_event;
mod announce_request;
mod router;
mod torrents;
// mod scrape;
mod peer;
mod info_hash;

fn main() {
    pretty_env_logger::init();
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let addr = (ip.to_string() + ":6969").parse().unwrap();
    let torrents = Arc::new(Mutex::new(Torrents::new()));

    let torrent_service = move || {
        let torrents = Arc::clone(&torrents);

        service_fn(move |req| {
            router::routes(req, Arc::clone(&torrents))
        })
    };

    let server = Server::bind(&addr)
        .serve(torrent_service)
        .map_err(|e| eprintln!("server error: {}", e));

    info!("Tracker running on {}...", addr);
    hyper::rt::run(server);
}
