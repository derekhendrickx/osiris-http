use std::sync::{Mutex};
use tracker::Tracker;

use hyper::Error;
use hyper::server::{Request, Response, Service};
use hyper::{Get, StatusCode};
use futures::future;
use futures::future::FutureResult;

use announce::Announce;
use scrape::Scrape;

pub struct Routes {
    tracker: Mutex<Box<Tracker>>
}

impl Routes {
    pub fn new() -> Routes {
        Routes {
            tracker: Mutex::new(Box::new(Tracker::new()))
        }
    }
}

impl Service for Routes {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = FutureResult<Response, Error>;

    fn call(&self, request: Request) -> Self::Future {
        future::ok(match (request.method(), request.path()) {
            (&Get, "/announce") => Announce::announce(&mut self.tracker.lock().unwrap(), &request),
            (&Get, "/scrape") => Scrape::scrape(&request),
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}