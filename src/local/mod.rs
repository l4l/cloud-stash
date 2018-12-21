use crate::chunk;
use crate::crypto::Hash;

pub mod memory;
#[cfg(feature = "persistent")]
pub mod sqlite;

#[derive(Debug)]
pub enum ErrorFind {
    /// File cannot be found
    NoMatch,
    /// There are similar names, which might be considered
    Matched(Vec<String>),
}

pub trait Db {
    fn save(&mut self, fname: &str, s: &[u8]) -> chunk::Chunks;
    // TODO: replace usize with metainfo
    fn find(&mut self, fname: &str) -> Result<(usize, Vec<Hash>), ErrorFind>;
    fn clean(&mut self, fname: &str);
    fn list(&mut self) -> Vec<(String, i64)>;
}
