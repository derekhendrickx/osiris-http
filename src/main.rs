#[macro_use]
extern crate bip_bencode;
extern crate byteorder;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate qstring;
extern crate tokio;
extern crate url;

use hyper::rt::Future;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::prelude::*;

use connection_info::ConnectionInfo;
use torrents::Torrents;

mod announce;
mod announce_event;
mod announce_request;
mod announce_response;
mod connection_info;
mod helper;
mod info_hash;
mod peer;
mod router;
mod scrape;
mod torrents;

fn main() {
    pretty_env_logger::init();
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let addr = (ip.to_string() + ":6969").parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();
    let http = Http::new();
    let torrents = Arc::new(Mutex::new(Torrents::new()));

    let server = listener
        .incoming()
        .for_each(move |socket| {
            let connection_info = ConnectionInfo::new(&socket);
            let torrents = Arc::clone(&torrents);

            let conn = http.serve_connection(
                socket,
                service_fn(move |mut req| {
                    connection_info.set(&mut req);
                    router::routes(&req, &torrents)
                }),
            );

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
