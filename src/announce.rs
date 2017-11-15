pub struct Announce;

impl Announce {
    pub fn announce(query: Option<&str>) {
        println!("Announce");
        match query {
            Some(str) => println!("Query: {:}", str),
            None => println!("Query: None"),
        }
    }
}
