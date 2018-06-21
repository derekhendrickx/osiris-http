use std::fmt;

use info_hash::InfoHash;

pub struct ScrapeRequest {
    info_hash: InfoHash,
}

impl ScrapeRequest {
    pub fn new(info_hash: &[u8]) -> ScrapeRequest {
        let info_hash = InfoHash::new(info_hash);

        ScrapeRequest { info_hash }
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

// impl fmt::Display for ScrapeRequest {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         writeln!(f, "\ninfo_hash = {}", self.info_hash)
//     }
// }
