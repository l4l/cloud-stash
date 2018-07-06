use sha3::{Sha3_256, Digest};

pub const HASH_SIZE: usize = 32;
pub type Hash = [u8; HASH_SIZE];

pub fn hash<'a>(s: &'a [u8]) -> Hash {
    let mut hasher = Sha3_256::default();
    hasher.input(s);
    let res = hasher.result();
    let mut hash = [0u8; HASH_SIZE];
    res.iter().enumerate().for_each(|(i, h)| hash[i] = *h);
    hash
}

pub fn hash_cmp(h1: &Hash, h2: &Hash) -> bool {
    for i in 0..HASH_SIZE {
        if h1[i] != h2[i] {
            return false;
        }
    }
    true
}

pub fn hash_hex(h: &Hash) -> String {
    h.iter().fold(
        String::with_capacity(HASH_SIZE * 2),
        |mut acc, b| {
            acc.push_str(&format!("{:x}", b));
            acc
        },
    )
}
