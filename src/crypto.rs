use sha3::{Sha3_256, Digest};

pub const HASH_SIZE: usize = 512;
pub type Hash = [u8; HASH_SIZE];

pub fn hash<'a>(s: &'a [u8]) -> Hash {
    let mut hasher = Sha3_256::default();
    hasher.input(s);
    let res = hasher.result();
    let mut hash = [0u8; HASH_SIZE];
    res.iter().enumerate().for_each(|(i, h)| hash[i] = *h);
    hash
}
