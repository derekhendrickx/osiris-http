pub struct Scrape;

impl Scrape {
	pub fn scrape(query: Option<&str>) {
		println!("Scrape");
		match query {
			Some(str) => println!("Query: {:}", str),
			None => println!("Query: None"),
		}
	}
}
