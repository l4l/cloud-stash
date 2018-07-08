use chunk;
use crypto::Hash;
pub mod sqlite;

#[derive(Debug)]
pub enum ErrorFind {
    /// File cannot be found
    NoMatch,
    /// There are similar names, which might be considered
    Matched(Vec<String>),
}

pub trait Db {
    fn save<'a>(&mut self, fname: &str, s: &'a [u8]) -> chunk::Chunks;
    // TODO: replace usize with metainfo
    fn find(&mut self, fname: &str) -> Result<(usize, Vec<Hash>), ErrorFind>;
    fn clean(&mut self, fname: &str);
}
