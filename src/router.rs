extern crate hyper;

use std::sync::{Arc, Mutex};

use torrents::Torrents;

use futures::future;
use hyper::header::{CACHE_CONTROL, CONTENT_TYPE};
use hyper::rt::Future;
use hyper::{Body, Method, Request, Response, StatusCode};

use announce::Announce;
use scrape::Scrape;

/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn routes(req: &Request<Body>, torrents: &Arc<Mutex<Torrents>>) -> BoxFut {
    let mut torrents = torrents.lock().unwrap();
    let mut response = Response::new(Body::empty());
    response
        .headers_mut()
        .insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    response
        .headers_mut()
        .insert(CONTENT_TYPE, "text/plain".parse().unwrap());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/announce") => {
            *response.body_mut() = Announce::announce(&mut torrents, &req)
        }

        (&Method::GET, "/scrape") => *response.body_mut() = Scrape::scrape(&mut torrents, &req),

        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Box::new(future::ok(response))
}
