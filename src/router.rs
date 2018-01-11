use std::sync::{Arc, Mutex};
use torrents::Torrents;

use hyper::Error;
use hyper::server::{Request, Response, Service};
use hyper::{Get, StatusCode};
use futures::future;
use futures::future::FutureResult;

use announce::Announce;
use scrape::Scrape;

pub struct Router {
    torrents: Arc<Mutex<Torrents>>,
}

impl Router {
    pub fn new(torrents: Arc<Mutex<Torrents>>) -> Router {
        Router { torrents }
    }
}

impl Service for Router {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = FutureResult<Response, Error>;

    fn call(&self, request: Request) -> Self::Future {
        let mut torrents = self.torrents.lock().unwrap();
        future::ok(match (request.method(), request.path()) {
            (&Get, "/announce") => Announce::announce(&mut torrents, &request),
            (&Get, "/scrape") => Scrape::scrape(&mut torrents, &request),
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}
