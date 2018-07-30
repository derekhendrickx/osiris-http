use hyper::{Body, Request};
use qstring::QString;

use helper::get_param_as_bytes;
use scrape_request::ScrapeRequest;
use torrents::Torrents;

pub struct Scrape;

impl Scrape {
    pub fn scrape(_torrents: &mut Torrents, request: &Request<Body>) -> Body {
        let scrape_request = Self::parse_request(request);

        Body::from(scrape_request.bencode())
    }

    fn parse_request(request: &Request<Body>) -> ScrapeRequest {
        let mut query_string = QString::from("");

        match request.uri().query() {
            Some(str) => query_string = QString::from(str),
            None => error!("Query: None"),
        }

        let info_hash = get_param_as_bytes(&query_string, "info_hash").unwrap();

        ScrapeRequest::new(&info_hash)
    }
}
