extern crate hyper;
extern crate futures;

use futures::future::Future;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

mod announce;

struct Tracker;

const PHRASE: &'static str = "Hello, World!";

impl Service for Tracker {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        println!("Method: {:}", req.method());
        println!("Path: {:}", req.path());

        match (req.method(), req.path()) {
            (&Method::Get, "/announce") => announce::Announce::announce(req.query()),
            (&Method::Get, "/scrape") => println!("Scrape"),
            _ => println!("{:}", StatusCode::NotFound),
        }

        match req.query() {
            Some(str) => println!("Query: {:}", str),
            None => println!("Query: None"),
        }

        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE)
        ))
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Tracker)).unwrap();
    server.run().unwrap();
}
