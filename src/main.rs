#[macro_use]
extern crate bip_bencode;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate url;

use std::sync::{Arc, Mutex};
use std::net::Ipv4Addr;

use hyper::server::Http;
use slog::*;

use torrents::Torrents;

mod router;
mod torrents;
mod announce;
mod scrape;
mod peer;
mod info_hash;

fn main() {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());

    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let addr = (ip.to_string() + ":6969").parse().unwrap();
    let torrents = Arc::new(Mutex::new(Torrents::new()));
    let server = Http::new()
        .bind(&addr, move || {
            let routes = router::Router::new(Arc::clone(&torrents));
            Ok(routes)
        })
        .unwrap();
    info!(logger, "Tracker started"; "ip" => format!("{}", addr));
    server.run().unwrap();
}
