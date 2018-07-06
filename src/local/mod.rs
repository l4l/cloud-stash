use chunk;
use crypto::Hashes;
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
    fn find(&mut self, fname: &str) -> Result<Hashes, ErrorFind>;
    fn clean(&mut self, fname: &str);
}
