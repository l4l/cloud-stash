use sha3::{Sha3_256, Digest};
use std::fmt;

pub const HASH_SIZE: usize = 32;
#[derive(Debug)]
pub struct Hash([u8; HASH_SIZE]);
pub type Hashes = Vec<Hash>;

impl Hash {
    pub fn new(h: [u8; HASH_SIZE]) -> Hash {
        Hash(h)
    }

    pub fn hash<'a>(&'a self) -> &'a [u8; HASH_SIZE] {
        &self.0
    }
}

pub fn hash<'a>(s: &'a [u8]) -> Hash {
    let mut hasher = Sha3_256::default();
    hasher.input(s);
    let res = hasher.result();
    let mut hash = [0u8; HASH_SIZE];
    res.iter().enumerate().for_each(|(i, h)| hash[i] = *h);
    Hash::new(hash)
}

impl PartialEq for Hash {
    fn eq(&self, other: &Hash) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0
            .iter()
            .map(|x| write!(f, "{:x}", x))
            .find(|r| r.is_err())
            .unwrap_or(Ok(()))
    }
}
