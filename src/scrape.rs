extern crate hyper;
extern crate qstring;

use std::fmt;
use hyper::{Request, Response, Body};
use hyper::header::{CACHE_CONTROL, CONTENT_TYPE};
use self::qstring::QString;

use torrents::Torrents;

pub struct Scrape;

struct ScrapeRequest {
    info_hash: String,
}

impl ScrapeRequest {
    fn new(data: &QString) -> ScrapeRequest {
        ScrapeRequest {
            info_hash: data.get("info_hash").unwrap().to_string(),
        }
    }

    fn bencode(self) -> Vec<u8> {
        let files = ben_map!{
            "complete" => ben_int!(1),
            "downloaded" => ben_int!(0),
            "incomplete" => ben_int!(0)
        };

        let message = ben_map!{
            "files" => files
        };

        message.encode()
    }
}

impl fmt::Display for ScrapeRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\ninfo_hash = {}", self.info_hash)
    }
}

impl Scrape {
    pub fn scrape(_torrents: &mut Torrents, request: &Request<Body>) -> Response<Body> {
        let mut query_string = QString::from("");

        match request.uri().query() {
            Some(str) => query_string = QString::from(str),
            None => error!("Query: None"),
        }

        let scrape_request = ScrapeRequest::new(&query_string);

        info!("Scrape Request: {:}", scrape_request);

        let body = scrape_request.bencode();

        let mut response = Response::new(Body::from(body));
        response.headers_mut().insert(CACHE_CONTROL, "no-cache".parse().unwrap());
        response.headers_mut().insert(CONTENT_TYPE, "text/plain".parse().unwrap());

        response
    }
}
