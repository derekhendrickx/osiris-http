#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InfoHash {
    hash: Vec<u8>,
}

impl InfoHash {
    pub fn new(hash: &[u8]) -> InfoHash {
        InfoHash { hash: hash.to_owned() }
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }
}
