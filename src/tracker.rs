extern crate hyper;
extern crate futures;

use hyper::server::{Request, Response, Service};
use hyper::{Get, StatusCode};
use self::futures::future::FutureResult;

use announce::{Announce};
use scrape::{Scrape};

pub struct Tracker;

impl Service for Tracker {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, request: Request) -> Self::Future {
        futures::future::ok(match (request.method(), request.path()) {
            (&Get, "/announce") => {
                Announce::announce(&request)
            },
            (&Get, "/scrape") => {
                Scrape::scrape(request.query())
            },
            _ => {
                Response::new().with_status(StatusCode::NotFound)
            }
        })
    }
}
