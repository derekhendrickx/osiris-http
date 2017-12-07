extern crate hyper;
extern crate qstring;

use std::fmt;
use hyper::server::{Request, Response};
use hyper::header::{Headers, ContentLength, ContentType, CacheControl, CacheDirective};
use hyper::mime;
use self::qstring::QString;

use tracker::Tracker;

pub struct Scrape;

struct ScrapeRequest {
    info_hash: String,
}

impl ScrapeRequest {
    fn new(data: &QString) -> ScrapeRequest {
        ScrapeRequest { info_hash: (&data["info_hash"]).to_string() }
    }

    fn bencode(self) -> Vec<u8> {
        let files =
            ben_map!{
            "complete" => ben_int!(1),
            "downloaded" => ben_int!(0),
            "incomplete" => ben_int!(0)
        };

        let message =
            ben_map!{
            "files" => files
        };

        message.encode()
    }
}

impl fmt::Display for ScrapeRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\ninfo_hash = {}\n", self.info_hash)
    }
}

impl Scrape {
    pub fn scrape(_tracker: &mut Tracker, request: &Request) -> Response {
        let mut query_string = QString::from("");

        match request.query() {
            Some(str) => query_string = QString::from(str),
            None => error!("Query: None"),
        }

        let scrape_request = ScrapeRequest::new(&query_string);

        info!("Scrape Request: {:}", scrape_request);

        let body = scrape_request.bencode();

        let mut headers = Headers::new();
        headers.set(ContentLength(body.len() as u64));
        headers.set(CacheControl(vec![CacheDirective::NoCache]));
        headers.set(ContentType(mime::TEXT_PLAIN));

        Response::new().with_headers(headers).with_body(body)
    }
}
