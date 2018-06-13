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
extern crate tokio;

use std::sync::{Arc, Mutex};
use std::net::Ipv4Addr;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::server::conn::Http;
use tokio::net::TcpListener;
use tokio::prelude::*;

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
    let listener = TcpListener::bind(&addr).unwrap();
    let http = Http::new();
    let torrents = Arc::new(Mutex::new(Torrents::new()));

    let server = listener.incoming().for_each(move |socket| {
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());
        let torrents = Arc::clone(&torrents);

        let conn = http.serve_connection(socket, service_fn(move |req| {
            router::routes(&req, &torrents)
        }));

        let fut = conn.map_err(|e| {
            eprintln!("server connection error: {}", e);
        });

        hyper::rt::spawn(fut);
        Ok(())
    })
    .map_err(|err| {
        // All tasks must have an `Error` type of `()`. This forces error
        // handling and helps avoid silencing failures.
        //
        // In our example, we are only going to log the error to STDOUT.
        println!("accept error = {:?}", err);
    });

    info!("Tracker running on {}...", addr);
    tokio::run(server);
}
