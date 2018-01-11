use std::sync::{Arc, Mutex};
use tracker::Tracker;

use hyper::Error;
use hyper::server::{Request, Response, Service};
use hyper::{Get, StatusCode};
use futures::future;
use futures::future::FutureResult;

use announce::Announce;
use scrape::Scrape;

pub struct Router {
    tracker: Arc<Mutex<Tracker>>,
}

impl Router {
    pub fn new(tracker: Arc<Mutex<Tracker>>) -> Router {
        Router { tracker }
    }
}

impl Service for Router {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = FutureResult<Response, Error>;

    fn call(&self, request: Request) -> Self::Future {
        let mut tracker = self.tracker.lock().unwrap();
        future::ok(match (request.method(), request.path()) {
            (&Get, "/announce") => Announce::announce(&mut tracker, &request),
            (&Get, "/scrape") => Scrape::scrape(&mut tracker, &request),
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}
