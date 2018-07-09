use sha3::{Sha3_256, Digest};
use std::fmt;
use std::slice::Iter;

pub const HASH_SIZE: usize = 32;
#[derive(Debug)]
pub struct Hash([u8; HASH_SIZE]);

impl Hash {
    pub fn new(h: [u8; HASH_SIZE]) -> Hash {
        Hash(h)
    }

    pub fn hash(&self) -> &[u8; HASH_SIZE] {
        &self.0
    }

    pub fn iter(&self) -> Iter<u8> {
        self.0.iter()
    }
}

pub fn hash(s: &[u8]) -> Hash {
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
            .map(|x| write!(f, "{:02x}", x))
            .find(|r| r.is_err())
            .unwrap_or(Ok(()))
    }
}

#[cfg(test)]
mod test {
    use crypto::{Hash, HASH_SIZE};
    #[test]
    fn test_hash_fmt() {
        let mut a = [0u8; HASH_SIZE];
        for i in 0..HASH_SIZE {
            a[i] = i as u8;
        }
        let h = Hash::new(a.clone());
        assert_eq!(
            format!("{}", h),
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        );
    }
}
