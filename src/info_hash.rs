#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InfoHash {
    hash: Vec<u8>,
}

impl InfoHash {
    pub fn new(hash: Vec<u8>) -> InfoHash {
        InfoHash { hash }
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }
}
