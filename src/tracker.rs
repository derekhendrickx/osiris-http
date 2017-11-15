extern crate hyper;
extern crate futures;

use self::futures::future::Future;

use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};
use hyper::{Method, StatusCode};

use announce::{Announce};
use scrape::{Scrape};

const PHRASE: &'static str = "Hello, World!";

pub struct Tracker;

impl Service for Tracker {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Get, "/announce") => Announce::announce(req.query()),
            (&Method::Get, "/scrape") => Scrape::scrape(req.query()),
            _ => println!("{:}", StatusCode::NotFound),
        }

        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE)
        ))
    }
}
