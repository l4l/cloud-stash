use chunk;
pub mod sqlite;

#[derive(Debug)]
pub enum ErrorFind {
    /// File cannot be found
    NoMatch,
    /// There are similar names, which might be considered
    Matched(Vec<String>),
}

pub trait Db {
    fn save<'a>(&mut self, _s: &'a [u8]) -> chunk::Chunks<'a>;
    fn find(&mut self, _fname: &str) -> Result<Vec<chunk::Hash>, ErrorFind>;
}
