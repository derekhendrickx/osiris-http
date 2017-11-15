extern crate hyper;
extern crate futures;

use hyper::server::{Request, Response, Service};
use hyper::{Method, StatusCode};
use self::futures::future::Future;

use announce::{Announce};
use scrape::{Scrape};

pub struct Tracker;

impl Service for Tracker {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
		let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/announce") => Announce::announce(req.query(), &mut response),
            (&Method::Get, "/scrape") => Scrape::scrape(req.query(), &mut response),
            _ => {
				response.set_status(StatusCode::NotFound);
			},
        }

        Box::new(futures::future::ok(response))
    }
}
